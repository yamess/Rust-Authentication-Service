use crate::repositories::auth_repository::AuthRepository;
use diesel::result::Error;
use diesel::{AsChangeset, Insertable, QueryDsl, Queryable, Selectable};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

// UserModel definition
#[derive(Insertable, Queryable, Selectable, Deserialize, Serialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UserRepository {
    pub id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub email: String,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub password: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable < diesel::sql_types::Timestamp >)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl UserRepository {
    pub async fn create(
        conn: &mut AsyncPgConnection,
        email: String,
        password: String,
    ) -> Result<Self, Error> {
        let hashed_password = AuthRepository::hash_password(&password).await;
        let user = Self {
            id: Uuid::new_v4(),
            email,
            password: hashed_password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };

        // let conn = &mut db.clone().pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .await
    }

    pub async fn get(conn: &mut AsyncPgConnection, id: Uuid) -> Result<Option<Self>, Error> {
        users::table
            .find(id)
            .get_result(conn)
            .await
            .map(Some)
            .or_else(|e| {
                if e == Error::NotFound {
                    log::warn!("User not found for id {}", id);
                    Ok(None)
                } else {
                    log::error!("Failed to get user: {}", e);
                    Err(e)
                }
            })
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: Uuid,
        email: String,
        password: String,
    ) -> Result<Self, Error> {
        let user = Self {
            id,
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: Some(chrono::Utc::now().naive_utc()),
        };

        diesel::update(users::table.find(id))
            .set(&user)
            .get_result(conn)
            .await
            .map_err(|e| {
                log::error!("Failed to update user: {}", e);
                e
            })
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: Uuid) -> usize {
        let num_deleted_rows = diesel::delete(users::table.find(id))
            .execute(conn)
            .await
            .map_err(|e| {
                log::error!("Failed to delete user: {}", e);
                e
            })
            .map(|num_deleted_rows| {
                log::info!("Deleted {} user(s)", num_deleted_rows);
                num_deleted_rows
            });
        num_deleted_rows.unwrap_or(0)
    }
}

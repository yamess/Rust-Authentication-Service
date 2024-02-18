use crate::schema::users;

use diesel::result::Error;
use diesel::{
    AsChangeset, Insertable, OptionalExtension, PgConnection, QueryDsl, Queryable, RunQueryDsl,
    Selectable,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl UserRepository {
    pub fn create(
        conn: &mut PgConnection,
        email: String,
        password: String,
    ) -> diesel::QueryResult<Self> {
        let user = Self {
            id: Uuid::new_v4(),
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };
        // let conn = &mut db.clone().pool.get().unwrap();
        Ok(diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .expect("Error creating user"))
    }

    pub fn get(conn: &mut PgConnection, id: Uuid) -> diesel::QueryResult<Option<Self>> {
        users::table.find(id).first(conn).optional()
    }

    pub fn update(&mut self, conn: &mut PgConnection) -> Option<Error> {
        self.updated_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(users::table.find(self.id))
            .set(&*self)
            .get_result::<Self>(conn)
            .ok();
        log::info!("User updated: {}", self.id);
        Some(Error::NotFound)
    }

    pub fn delete(conn: &mut PgConnection, id: Uuid) -> bool {
        diesel::delete(users::table.find(id))
            .execute(conn)
            .expect("Error deleting user");
        true
    }
}

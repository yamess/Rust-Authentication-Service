use crate::database::postgres::PostgresConnectionPool;
use crate::schema::users;
use diesel::associations::HasTable;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::{AsChangeset, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable};
use r2d2::Pool;
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
    pub fn create(db: &PostgresConnectionPool, email: String, password: String) -> Self {
        let user = Self {
            id: Uuid::new_v4(),
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        };
        let conn = &mut db.clone().pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .expect("Error creating user")
    }

    pub fn get(db: &PostgresConnectionPool, id: Uuid) -> Option<Self> {
        let conn = &mut db.clone().pool.get().unwrap();
        users::table.find(id).first(conn).ok()
    }

    pub fn update(&mut self, db: &PostgresConnectionPool) -> Option<Error> {
        let conn = &mut db.clone().pool.get().unwrap();
        self.updated_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(users::table.find(self.id))
            .set(&*self)
            .get_result::<Self>(conn)
            .ok();
        log::info!("User updated: {}", self.id);
        Some(Error::NotFound)
    }

    pub fn delete(&self, db: &PostgresConnectionPool) -> bool {
        let conn = &mut db.clone().pool.get().unwrap();
        diesel::delete(users::table.find(self.id))
            .execute(conn)
            .expect("Error deleting user");
        true
    }
}

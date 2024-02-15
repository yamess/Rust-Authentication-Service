use crate::interfaces::database_interface::IDatabase;
use std::arch::aarch64::ST;
use std::fmt::Debug;

use crate::database::models::UserModel;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use serde::{Deserialize, Serialize};

pub struct PostgresDatabase {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDatabase {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        PostgresDatabase { pool }
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        self.pool.get()
    }
}

impl<T> IDatabase<T> for PostgresDatabase
where
    T: 'static
        + Send
        + Sync
        + Clone
        + diesel::Insertable<T>
        + diesel::AsChangeset
        + diesel::Identifiable
        + diesel::Queryable<T, diesel::pg::Pg>
        + diesel::Selectable<diesel::pg::Pg>
        + Serialize
        + Deserialize
        + Debug,
{
    fn create(&self, entity: &T) -> T {
        let conn = self.get_connection().unwrap();
        entity.insert_into(T::table_name()).values(entity).get_result(&conn).unwrap()
    }

    fn read(&self, id: T) -> T {
        let conn = self.get_connection().unwrap();
        T::find_by_id(&id, &conn).unwrap()
    }

    fn update(&self, entity: T) -> T {
        let conn = self.get_connection().unwrap();
        entity.update(&conn)
    }

    fn delete(&self, id: T) -> T {
        let conn = self.get_connection().unwrap();
        T::delete_by_id(&id, &conn).unwrap()
    }
}

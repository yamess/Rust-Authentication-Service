use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub struct PostgresConnectionPool {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresConnectionPool {
    fn new(database_url: &str, size: u32) -> Self {
        let pool = Pool::builder()
            .max_size(size)
            .test_on_check_out(true)
            .build(ConnectionManager::new(database_url))
            .expect("Failed to create pool");
        log::info!("Postgres connection pool created");
        PostgresConnectionPool { pool }
    }

    /*
        fn create(&mut self, entity: &T) -> T {
            diesel::insert_into(T::table())
                .values(entity)
                .get_result::<T>(&mut self.conn)
                .expect("Error creating user")
        }
    */
    /*
    fn get(&self, id: String) -> Option<T> {
        let conn = self.pool.get().unwrap();
        let _id = Uuid::parse_str(&id).unwrap();
        T::table()
            .find(_id)
            .load::<T>(&conn)
            .expect("Error loading users")
    }

    fn update(&self, id: String, entity: T) -> T {
        let mut conn = &self.pool.get().unwrap();
        let _id = Uuid::parse_str(&id).unwrap();
        diesel::update(entity.table().find(_id))
            .set(&entity)
            .get_result(conn)
            .expect("Error updating user")
    }

    fn delete(&self, id: String, entity: T) -> bool {
        let mut conn = &self.pool.get().unwrap();
        let _id = Uuid::parse_str(&id).unwrap();
        diesel::delete(T::table().find(_id))
            .execute(conn)
            .expect("Error deleting user");
        true
    }

     */
}

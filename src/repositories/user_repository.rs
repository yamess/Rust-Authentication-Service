use crate::database::postgres::PostgresConnectionPool;
use crate::interfaces::repository_interface::IRepository;
use crate::models::user_model::UserModel;
use crate::schema::users::dsl::users;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;

pub struct UserRepository {
    conn_pool: Pool<ConnectionManager<PgConnection>>,
}

impl IRepository for UserRepository {
    type Model = UserModel;
    type Connection = PostgresConnectionPool;
    fn new(connection_pool: PostgresConnectionPool) -> Self {
        let conn = connection_pool.pool.clone();
        UserRepository { conn_pool: conn }
    }

    fn create(&mut self, model: Self::Model) -> Self::Model {
        let conn = &mut self.conn_pool.get().unwrap();
        diesel::insert_into(users)
            .values(&model)
            .get_result(conn)
            .expect("Error creating user")
    }
}

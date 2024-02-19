use bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log;

use crate::settings::configs::DatabaseConfig;

#[derive(Clone, Debug)]
pub struct AsyncPostgresConnectionPool {
    pub pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl AsyncPostgresConnectionPool {
    pub async fn new(database_config: &DatabaseConfig) -> Self {
        let database_url = &database_config.database_url;
        let pool_size = &database_config.pool_size;
        let db_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

        let pool = bb8::Pool::builder()
            .max_size(*pool_size)
            .test_on_check_out(true)
            .retry_connection(true)
            .build(db_manager)
            .await
            .expect("Failed to create pool");
        log::info!("Postgres connection pool created");
        AsyncPostgresConnectionPool { pool }
    }
}

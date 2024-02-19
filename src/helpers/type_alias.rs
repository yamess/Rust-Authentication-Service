use bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
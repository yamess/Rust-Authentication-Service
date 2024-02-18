use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
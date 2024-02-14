use diesel::PgConnection;
use uuid::Uuid;

pub trait  IRepository<T> {
    fn create(&self, conn: &mut PgConnection) -> T;
    fn find_by_id(id: &Uuid, conn: &mut PgConnection) -> Option<T>;
    fn get(id: &Uuid, conn: &mut PgConnection) -> Vec<T>;
    fn update(&self, conn: &mut PgConnection) -> T;
}

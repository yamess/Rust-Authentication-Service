pub trait IRepository {
    type Model;
    type Connection;
    fn new(conn: Self::Connection) -> Self;
    fn create(&mut self, model: Self::Model) -> Self::Model;
}

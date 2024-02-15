pub trait IDatabase<T: 'static> {
    fn connect(&self) -> bool;
    fn disconnect(&self) -> bool;
    fn query(&self, query: &str) -> bool;
    fn create(&self, entity: T) -> T;
    fn read(&self, id: T) -> T;
    fn update(&self, entity: T) -> T;
    fn delete(&self, id: T) -> T;
}
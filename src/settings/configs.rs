use envy::from_env;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub database_url: String,
}

pub struct GlobalConfig {
    pub database: DatabaseConfig,
}

impl GlobalConfig {
    pub fn new() -> Self {
        let database: DatabaseConfig = from_env::<DatabaseConfig>().unwrap();
        GlobalConfig { database }
    }
}

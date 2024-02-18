mod database;
mod helpers;
mod repositories;
mod schema;
mod server;
mod settings;

use crate::database::postgres::PostgresConnectionPool;
use chrono::Utc;
use diesel::{Connection, PgConnection};
use env_logger;
use fern::colors::{Color, ColoredLevelConfig};
use helpers::logger::*;
use log::info;
use repositories::user_repository::UserRepository;
use settings::configs::GlobalConfig;

use uuid::Uuid;

fn main() {
    setup_logger("output.txt");
    let config = GlobalConfig::new();
    let database = PostgresConnectionPool::new(&config.database);

    // Create a new user
    /*
    let user_repo = UserRepository::create(
        &database,
        String::from("yamess@gmail.com"),
        String::from("password1234"),
    );
    log::info!("{:?}", user_repo);
    */

    // Get user by id
    let user_id = Uuid::parse_str("99984e30-04d3-4a43-9209-73b5a0d34980").unwrap();
    let mut user = match UserRepository::get(&database, user_id) {
        Some(user) => user,
        None => {
            log::info!("User {} not found", user_id);
            return;
        }
    };
    log::info!("Retrieved user: {:?}", user);

    // Update user
    user.email = String::from("babdongo@yahoo.fr");
    user.update(&database).expect("Error updating user");
    log::info!("Updated user: {:?}", user);

    /*
     let connection = &mut create_connection(&config.database.database_url);

     let new_user = UserModel::new(String::from("test@rust.com"), String::from("password1234"));
     // Create a new user
     let created_user = new_user.create(connection);
     println!("New Use created: {:?}", created_user);

     // Read users

     let user = UserModel::find_by_id(
         &Uuid::parse_str("25c49b87-ad12-4ffe-85f3-3bad6d4f05d8").unwrap(),
         connection,
     );
     let user = match user {
         Some(user) => user,
         None => {
             println!("User not found");
             return;
         }
     };
     // Update user
     /*
     let mut _user = &mut users[0];
      _user.password = String::from("new_password");
     let updated_user = _user.update(connection);
     println!("{:?}", updated_user);
     */

     // Delete User

     let deleted_user = user.delete(connection);
     println!("{:?}", deleted_user);

     //println!("{:?}", new_user);
     //println!("{:?}", created_user);
    */
}

mod database;
mod helpers;
mod interfaces;
mod repositories;
mod schema;
mod server;
mod settings;

use database::models::UserModel;
use diesel::{Connection, PgConnection};
use settings::configs::GlobalConfig;
use uuid::Uuid;

fn create_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let config = GlobalConfig::new();
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
}

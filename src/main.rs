mod schema;

use envy::from_env;
use diesel::{Connection, Insertable, PgConnection, Queryable, RunQueryDsl, Selectable};
use schema::users;
use serde::{Deserialize, Serialize};
use uuid;


// Models definitions
#[derive(Queryable, Deserialize, Selectable, Serialize, Debug)]
#[diesel(table_name = users)]
struct User {
    id: uuid::Uuid,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    email: String,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    password: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
struct NewUser {
    id: uuid::Uuid,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    email: String,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    password: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    created_at: chrono::NaiveDateTime,
}
impl NewUser {
    fn new(email: String, password: String) -> NewUser {
        NewUser {
            id: uuid::Uuid::new_v4(),
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
struct DatabaseConfig {
    database_url: String,
}

fn create_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let config = from_env::<DatabaseConfig>().unwrap();
    let connection = &mut create_connection(&config.database_url);

    let new_user = NewUser::new(
        String::from("test@rust.com"),
        String::from("my_password"),
    );

    let created_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user");

    println!("{:?}", new_user);
    println!("{:?}", created_user);
}

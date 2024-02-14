mod schema;

use diesel::{
    AsChangeset, Connection, Insertable, OptionalExtension, PgConnection, QueryDsl, Queryable,
    RunQueryDsl, Selectable, SelectableHelper,
};
use envy::from_env;
use schema::users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Models definitions
#[derive(Queryable, Insertable, Selectable, Deserialize, Serialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
struct User {
    pub id: Uuid,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub email: String,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub password: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl User {
    fn new(email: String, password: String) -> User {
        User {
            id: Uuid::new_v4(),
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    fn create(&self, conn: &mut PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }

    fn find_by_id(id: &Uuid, conn: &mut PgConnection) -> Option<User> {
        users::table
            .find(id)
            .first(conn)
            .optional()
            .expect("Error loading users")
    }

    fn get(id: &Uuid, conn: &mut PgConnection) -> Vec<User> {
        users::table
            .find(id)
            .load::<User>(conn)
            .expect("Error loading users")
    }

    fn update(&self, conn: &mut PgConnection) -> User {
        let updated_user = User {
            id: self.id,
            email: self.email.clone(),
            password: self.password.clone(),
            created_at: self.created_at,
            updated_at: Some(chrono::Utc::now().naive_utc()),
        };
        diesel::update(users::table.find(self.id))
            .set(&updated_user)
            .get_result(conn)
            .expect("Error updating user")
    }

    fn delete(&self, conn: &mut PgConnection) -> usize {
        diesel::delete(users::table.find(self.id))
            .execute(conn)
            .expect("Error deleting user")
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
    /*
        let new_user = User::new(
            String::from("test@rust.com"),
            String::from("password1234"),
        );
    */
    // Create a new user
    /*
    let created_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user");
    */

    /*
    let created_user = new_user.create(connection);
    println!("{:?}", created_user);
    */

    // Read users

    let user = User::find_by_id(
        &Uuid::parse_str("25c49b87-ad12-4ffe-85f3-3bad6d4f05d8").unwrap(),
        connection,
    );
    match user {
        Some(user) => println!("{:?}", user),
        None => println!("User not found"),
    }
    // Update user
    /*
    let mut _user = &mut users[0];
     _user.password = String::from("new_password");
    let updated_user = _user.update(connection);
    println!("{:?}", updated_user);
    */

    // Delete User
    /*
    let user = &users[0];
    let deleted_user = user.delete(connection);
    println!("{:?}", deleted_user);
    */

    //println!("{:?}", new_user);
    //println!("{:?}", created_user);
}

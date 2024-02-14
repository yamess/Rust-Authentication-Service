mod schema;

use envy::from_env;
use diesel::{AsChangeset, Connection, Insertable, PgConnection, Queryable, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use schema::users;
use serde::{Deserialize, Serialize};
use uuid;


// Models definitions
#[derive(Queryable, Deserialize, Selectable, Serialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
struct User {
    pub id: uuid::Uuid,
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
    fn read(conn: &mut PgConnection) -> Vec<User> {
        users::table
            .load::<User>(conn)
            .expect("Error loading users")
    }

    fn update(&self, conn: &mut PgConnection) -> User {
        diesel::update(users::table.find(self.id))
            .set(self)
            .get_result(conn)
            .expect("Error updating user")
    }

    fn delete(&self, conn: &mut PgConnection) -> User {
        diesel::delete(users::table.find(self.id))
            .get_result(conn)
            .expect("Error deleting user")
    }
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

    fn create(&self, conn: &mut PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
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

    // Create a new user
    /*
    let created_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user");
    */
    // let created_user = new_user.create(connection);

    // Read users
    let mut users = User::read(connection);
    for user in &users {
        println!("{:?}", user);
    }

    let mut _user = &mut users[0];
     _user.password = String::from("new_password");
    let updated_user = _user.update(connection);
    println!("{:?}", updated_user);


    // Update user


    //println!("{:?}", new_user);
    //println!("{:?}", created_user);
}

use crate::schema::users;
use diesel::{
    AsChangeset, Insertable, OptionalExtension, PgConnection, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Deserialize, Serialize, AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UserModel {
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

impl UserModel {
    pub fn new(email: String, password: String) -> UserModel {
        UserModel {
            id: Uuid::new_v4(),
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> UserModel {
        diesel::insert_into(users::table)
            .values(self)
            .returning(UserModel::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }

    pub fn find_by_id(id: &Uuid, conn: &mut PgConnection) -> Option<UserModel> {
        users::table
            .find(id)
            .first(conn)
            .optional()
            .expect("Error loading users")
    }

    pub fn get(id: &Uuid, conn: &mut PgConnection) -> Vec<UserModel> {
        users::table
            .find(id)
            .load::<UserModel>(conn)
            .expect("Error loading users")
    }

    pub fn update(&self, conn: &mut PgConnection) -> UserModel {
        let updated_user = UserModel {
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

    pub fn delete(&self, conn: &mut PgConnection) -> usize {
        diesel::delete(users::table.find(self.id))
            .execute(conn)
            .expect("Error deleting user")
    }
}

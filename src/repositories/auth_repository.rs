use crate::repositories::user_repository::UserRepository;
use crate::schema::users;
use diesel::deserialize::FromSql;
use diesel::expression_methods::ExpressionMethods;
use diesel::pg::Pg;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::collections::HashMap;

pub struct AuthRepository;

impl AuthRepository {
    pub async fn hash_password(password: &str) -> String {
        bcrypt::hash(password, 12).unwrap()
    }

    pub async fn verify_password(password: &str, hash: &str) -> bool {
        bcrypt::verify(password, hash).unwrap()
    }

    pub async fn login(
        conn: &mut AsyncPgConnection,
        email: &str,
        password: &str,
    ) -> Result<UserRepository, Error> {
        let user = users::table
            .filter(users::email.eq(email))
            .get_result::<UserRepository>(conn)
            .await?;
        if Some(user.id) == None {
            log::error!("User not found for email {}", email);
            return Err(Error::NotFound);
        }
        if AuthRepository::verify_password(password, &user.password).await {
            Ok(user)
        } else {
            log::error!("Wrong credentials for user {}", email);
            Err(Error::NotFound)
        }
    }
}

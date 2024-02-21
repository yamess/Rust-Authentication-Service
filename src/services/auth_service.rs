use crate::helpers::type_alias::DbPool;
use crate::repositories::auth_repository::AuthRepository;
use actix_web::{error, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Login {
    email: String,
    password: String,
}

#[post("/auth/login")]
async fn login(
    auth_data: web::Json<Login>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let auth_data = auth_data.into_inner();
    let mut async_pool = pool.get().await.expect("Failed to get pool");

    let user = AuthRepository::login(&mut async_pool, &auth_data.email, &auth_data.password)
        .await
        .map_err(|e| {
            log::error!("Failed to login: {}", e);
            error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(user))
}

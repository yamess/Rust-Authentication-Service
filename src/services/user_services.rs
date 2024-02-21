use actix_web::{delete, error, get, HttpResponse, post, Responder, web};
use uuid::Uuid;

use crate::helpers::type_alias::DbPool;
use crate::repositories::user_repository::UserRepository;
use crate::schemas::user_schema::CreateUser;

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> actix_web::Result<impl Responder> {
    let user_data = user.into_inner();
    let mut async_pool = pool
        .get()
        .await
        .expect("Failed to get pool");

    let created_user =
        UserRepository::create(&mut async_pool, user_data.email, user_data.password)
            .await
            .map_err(|e| {
                log::error!("Failed to create user: {}", e);
                error::ErrorInternalServerError(e)
            })?;

    Ok(HttpResponse::Ok().json(created_user))
}

#[get("/users/{id}")]
async fn get_users(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let id = id.into_inner();
    log::info!("retrieving user {} data", id);

    let mut async_pool = pool
        .get()
        .await
        .expect("Failed to get pool");

    let user = UserRepository::get(&mut async_pool, id)
        .await
        .map_err(|e| {
            log::error!("Failed to get user: {}", e);
            error::ErrorInternalServerError(e)
        })?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/users/{id}")]
async fn delete_user(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let id = id.into_inner();
    log::info!("deleting user {} data", id);

    let mut async_pool = pool
        .get()
        .await
        .map_err(|e| {
            log::error!("Failed to get pool: {}", e);
            error::ErrorInternalServerError(e)
        })?;

    let num_deleted_row = UserRepository::delete(&mut async_pool, id)
        .await;
    match num_deleted_row {
        1 => Ok(HttpResponse::Ok().finish()),
        _ => Ok(HttpResponse::NotFound().finish())
    }
    //Ok(HttpResponse::Ok().json(user))
}


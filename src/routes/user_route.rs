use crate::helpers::type_alias::DbPool;
use crate::repositories::user_repository::UserRepository;
use crate::schemas::user_schema::{CreateUser, UpdateUser};
use actix_web::{error, get, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> actix_web::Result<impl Responder> {
    let user_data = user.into_inner();
    let created_user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        UserRepository::create(&mut conn, user_data.email, user_data.password)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(created_user))
}

#[get("/users/{id}")]
async fn get_users(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let id = id.into_inner();
    log::info!("retrieving user {} data", id);
    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        UserRepository::get(&mut conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

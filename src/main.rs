use actix_web::{dev::Service, middleware, web, App, HttpMessage, HttpResponse, HttpServer};
use chrono::Utc;

use database::postgres::AsyncPostgresConnectionPool;
use helpers::logger::*;
use routes::user_route::{create_user, delete_user, get_users};
use settings::configs::GlobalConfig;

mod database;
mod helpers;
mod middlewares;
mod repositories;
mod routes;
mod schema;
mod schemas;
mod server;
mod services;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_name = format!("logs/{}.log", Utc::now().format("%Y-%m-%d"));
    let _ = setup_logger(&file_name);
    let config = GlobalConfig::new();
    log::info!("Starting server...");

    let database = AsyncPostgresConnectionPool::new(&config.database).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(web::Data::new(database.pool.clone()))
            .route(
                "/health",
                web::to(|| async { HttpResponse::Ok().body("OK") }),
            )
            .service(create_user)
            .service(get_users)
            .service(delete_user)
    })
    //.workers(5)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

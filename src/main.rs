mod database;
mod helpers;
mod repositories;
mod routes;
mod schema;
mod schemas;
mod server;
mod settings;

use crate::database::postgres::PostgresConnectionPool;
use crate::routes::user_route::get_users;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use helpers::logger::*;
use routes::user_route::create_user;
use settings::configs::GlobalConfig;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = setup_logger("output.txt");
    let config = GlobalConfig::new();
    log::info!("Starting server...");

    let database = PostgresConnectionPool::new(&config.database);

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
    })
    //.workers(5)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

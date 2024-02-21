use actix_web::{dev::Service, middleware, web, App, HttpMessage, HttpResponse, HttpServer};
use chrono::Utc;
use std::sync::Arc;

use database::postgres::AsyncPostgresConnectionPool;
use helpers::logger::*;
use middlewares::timer::TimerMiddleware;
use services::user_services::{create_user, delete_user, get_users};
use settings::configs::GlobalConfig;

mod database;
mod helpers;
mod middlewares;
mod repositories;
mod schema;
mod schemas;
mod server;
mod services;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_name = format!("logs/{}.log", Utc::now().format("%Y-%m-%d"));

    match setup_logger(&file_name) {
        Ok(_) => log::info!("Logger initialized"),
        Err(e) => {
            log::error!("Failed to initialize logger: {}", e);
            panic!("Failed to initialize logger")
        }
    }
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
            .wrap(TimerMiddleware)
            .wrap_fn(|req, serv| {
                let start = std::time::Instant::now();
                let fut = serv.call(req);
                async move {
                    let res = fut.await?;
                    let elapsed = start.elapsed();
                    log::info!(
                        "Elapsed time from second middleware {}",
                        elapsed.as_millis().to_string().parse::<String>().unwrap()
                    );
                    Ok(res)
                }
            })
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

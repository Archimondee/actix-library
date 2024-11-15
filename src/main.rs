mod db;
mod handlers;
mod models;
mod queries;
mod schema;
mod utils;

use actix_web::{web, App, HttpServer};
use db::establish_connection;

use handlers::health_handlers::configure_health_handlers;
use log::info;
use simplelog::{Config, SimpleLogger};

fn init_logger() {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default())
        .expect("Failed to initialize logger");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    info!("Application is starting up...");
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(utils::cors::cors())
            .wrap(utils::logger::logger())
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/v1").configure(configure_health_handlers))
            .default_service(web::route().to(utils::not_found::not_found))
    })
    .bind("127.0.0.1:5100")?
    .run()
    .await
}

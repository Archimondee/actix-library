mod db;
mod handlers;
mod models;
mod queries;
mod schema;
mod utils;

use std::time::Duration;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    web::{self, JsonConfig},
    App, HttpServer,
};
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
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_minute(60)
        .period(Duration::from_secs(60))
        .burst_size(20)
        .finish()
        .unwrap();
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(utils::logger())
            .wrap(utils::cors())
            .app_data(web::Data::new(pool.clone()))
            .app_data(JsonConfig::default().limit(4096 * 1024))
            .wrap(Governor::new(&governor_conf))
            .service(web::scope("/v1").configure(configure_health_handlers))
            .default_service(web::route().to(utils::not_found))
    })
    .bind("127.0.0.1:5100")?
    .run()
    .await
}

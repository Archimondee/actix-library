mod db;
mod handlers;
mod models;
mod queries;
mod schema;

use actix_web::{middleware::Logger, web, App, HttpServer};
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
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("%a %T \"%r\" %s %b %Dms"))
            .service(web::scope("/v1").configure(configure_health_handlers))
    })
    .bind("127.0.0.1:5100")?
    .run()
    .await
}

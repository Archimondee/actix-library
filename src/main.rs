mod db;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use db::establish_connection;
use log::info;
//use serde::Serialize;
use simplelog::{Config, SimpleLogger};

fn init_logger() {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default())
        .expect("Failed to initialize logger");
}

// #[derive(Serialize)]
// struct ErrorResponse {
//     error: String,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    info!("Application is starting up...");
    let pool = establish_connection();

    HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())))
        .bind("127.0.0.1:5100")?
        .run()
        .await
}

use std::{env, io};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use db::establish_connection_pool;
use env_logger::Env;
use log::{log, Level};

pub mod models;
pub mod schema;
pub mod db;
pub mod service;
pub mod handlers;

const LOGGER_FORMAT: &str = "[%t] %a %s UA:%{User-Agent}i CT:%{Content-Type}i %Dms";

/// print all erorrs and contexts inside the chain
pub fn unroll_anyhow_result(e: anyhow::Error) -> String {
    let mut res = String::new();
    for (i, small_e) in e.chain().enumerate() {
        res.push_str(&format!("{}{}\n", "\t".repeat(i), small_e));
    }
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_url = env::var("DB_URL");
    if db_url.is_err() {
        log!(Level::Error, "Getting DB_URL env var value: {}", db_url.err().unwrap());
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing environment variable: DB_URL"));
    }
    let db_url = db_url.unwrap();

    let connection_pool = establish_connection_pool(db_url);
    if connection_pool.is_err() {
        let err_msg = unroll_anyhow_result(connection_pool.err().unwrap());
        log!(Level::Error, "Creating DB connection pool: {}", err_msg);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Error creating DB connection pool: {err_msg}")));
    }
    let connection_pool = connection_pool.unwrap();
    log!(Level::Info, "DB connection pool created");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(LOGGER_FORMAT))
            .wrap(Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "OPTIONS"]))
            .app_data(Data::new(connection_pool.clone()))
            .service(handlers::blogpost_handler::create_blogpost)
            .service(handlers::blogpost_handler::get_feed)
            .service(handlers::image_handler::get_image)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

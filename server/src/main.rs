use std::{env, io};
use actix_web::{middleware::Logger, App, HttpServer};
use db::establish_connection_pool;
use env_logger::Env;
use log::{log, Level};

pub mod models;
pub mod schema;
pub mod db;
pub mod service;

const LOGGER_FORMAT: &str = "[%t] %a %s UA:%{User-Agent}i CT:%{Content-Type}i %Dms";

/// unroll all contexts and error inside the error chain
fn unroll_anyhow_result(e: anyhow::Error) -> String {
    let mut res = String::new();
    for (i, small_e) in e.chain().enumerate().skip(1) {
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
        log!(Level::Error, "Creating DB connection pool: {err_msg}");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Error creating DB connection pool: {err_msg}"));
    }
    log!(Level::Info, "DB connection pool created");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(LOGGER_FORMAT))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

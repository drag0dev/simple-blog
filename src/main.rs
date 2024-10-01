use std::{env, io};
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use log::{log, Level};

pub mod models;
pub mod schema;

const LOGGER_FORMAT: &str = "[%t] %a %s UA:%{User-Agent}i CT:%{Content-Type}i %Dms";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_url = env::var("DB_URL");
    if db_url.is_err() {
        log!(Level::Error, "getting DB_URL env var value: {}", db_url.err().unwrap());
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing environment variable: DB_URL"));
    }

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(LOGGER_FORMAT))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

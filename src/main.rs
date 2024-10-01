use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

const LOGGER_FORMAT: &str = "[%t] %a %s UA:%{User-Agent}i CT:%{Content-Type}i %Dms";

// TODO: pass port as an env var

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(LOGGER_FORMAT))
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}

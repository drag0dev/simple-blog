#[cfg(test)]
mod tests {
    use std::env;

    use actix_web::{test, web::Data, App};
    use tokio::fs;
    use uuid::Uuid;
    use crate::{db::establish_connection_pool, handlers::image_handler::get_image};

    #[actix_web::test]
    async fn test_placeholder_avatar() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/image/placeholder_avatar")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_nonexistant_image() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(get_image)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/image/random_string_")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_existant_image() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(get_image)
        ).await;

        let uuid = Uuid::new_v4().to_string();
        fs::copy("images/placeholder_avatar", format!("images/{uuid}"))
            .await
            .expect("making a dummy image");

        let req = test::TestRequest::get()
            .uri(&format!("/api/v1/image/{uuid}"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

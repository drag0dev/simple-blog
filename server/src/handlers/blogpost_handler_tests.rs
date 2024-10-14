#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::File,
        io::{Read, Write}
    };
    use actix_web::{body::MessageBody, test, web::{self, Data}, App};
    use anyhow::Result;
    use serde_json::{from_str, to_string};
    use crate::{
        db::establish_connection_pool,
        handlers::blogpost_handler::{create_blogpost, get_feed},
        models::{CreateBlogPostDTO, FeedDTO}};
    use diesel::{PgConnection, RunQueryDsl};

    /// TESTS NEED TO BE RAN SEQUENTIALLY

    /// helper function to manually construct a multipart form payload
    fn create_multipart(dto: String, image_file: &mut File) -> Vec<u8> {
        let boundary = "my_boundary";
        let mut body = Vec::new();

        write!(
            &mut body,
            "--{}\r\n\
            Content-Disposition: form-data; name=\"data\"\r\n\r\n\
        {}\r\n",
        boundary, dto).unwrap();

        let mut image_data = Vec::new();
        image_file.read_to_end(&mut image_data).expect("Failed to read image file");

        write!(
            &mut body,
            "--{}\r\n\
            Content-Disposition: form-data; name=\"image\"; filename=\"image.png\"\r\n\
        Content-Type: image/png\r\n\r\n",
        boundary).unwrap();
        body.extend(image_data);
        body.write_all(b"\r\n").unwrap();

        write!(
            &mut body,
            "--{}--\r\n",
            boundary).unwrap();

        body
    }

    fn delete_all_posts(conn: &mut PgConnection) -> Result<()> {
        use crate::schema::blogpost::blogpost::table as BlogpostTable;
        diesel::delete(BlogpostTable).execute(conn)?;
        Ok(())
    }


    #[actix_web::test]
    async fn test_no_blogspots_present() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");
        let mut conn = connection_pool.get().expect("getting connection");

        let _ = web::block(move || { delete_all_posts(&mut conn) })
            .await
            .expect("deleting all posts");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(get_feed)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/blogpost?page=1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = resp
            .into_body()
            .try_into_bytes()
            .expect("reading response bytes")
            .to_vec();
        let body = String::from_utf8(body_bytes).expect("reading response bytes as string");
        let unescaped_body = from_str::<String>(&body).expect("reading unsecaped body");
        let feed: FeedDTO = from_str(&unescaped_body).expect("parsing feed body");
        assert!(feed.blogposts.len() == 0);
    }

    #[actix_web::test]
    async fn test_missing_page_param() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(get_feed)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/blogpost")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_invalid_page_param() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(get_feed)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/blogpost?page=-1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_valid_blogpost_creation() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(create_blogpost)
            .service(get_feed)
        ).await;

        let dto = CreateBlogPostDTO {
            text: "Hello!".to_string(),
            username: "admin".to_string(),
            avatar: Some("https://w7.pngwing.com/pngs/114/579/png-transparent-pink-cross-stroke-ink-brush-pen-red-ink-brush-ink-leave-the-material-text.png".to_string()),
        };
        let dto_str = to_string(&dto).expect("turning dto to json string");

        let mut image_file = File::open(format!("images/placeholder_avatar"))
            .expect("opening placeholder avatar");

        let form = web::block(move || {create_multipart(dto_str, &mut image_file)})
            .await.unwrap();

        let req = test::TestRequest::post()
            .set_payload(form)
            .insert_header(("Content-Type", "multipart/form-data; boundary=my_boundary"))
            .uri("/api/v1/blogpost")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // check if the created post is present
        let req = test::TestRequest::get()
            .uri("/api/v1/blogpost?page=1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = resp
            .into_body()
            .try_into_bytes()
            .expect("reading response bytes")
            .to_vec();
        let body = String::from_utf8(body_bytes).expect("reading response bytes as string");
        let unescaped_body = from_str::<String>(&body).expect("reading unsecaped body");
        let feed: FeedDTO = from_str(&unescaped_body).expect("parsing feed body");
        assert!(feed.blogposts.len() == 1);
        assert_eq!(feed.blogposts[0].text, "Hello!");
        assert_eq!(feed.blogposts[0].username, "admin");
        assert!(feed.blogposts[0].avatar.is_some());
    }

    #[actix_web::test]
    async fn test_invalid_blogpost_malformed_data() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let mut conn = connection_pool.get().expect("getting connection");

        let _ = web::block(move || { delete_all_posts(&mut conn) })
            .await
            .expect("deleting all posts");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(create_blogpost)
            .service(get_feed)
        ).await;

        let mut image_file = File::open(format!("images/placeholder_avatar"))
            .expect("opening placeholder avatar");

        let form = web::block(move || {create_multipart("test".to_string(), &mut image_file)})
            .await.unwrap();

        let req = test::TestRequest::post()
            .set_payload(form)
            .insert_header(("Content-Type", "multipart/form-data; boundary=my_boundary"))
            .uri("/api/v1/blogpost")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());

        // check if the created post is present
        let req = test::TestRequest::get()
            .uri("/api/v1/blogpost?page=1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = resp
            .into_body()
            .try_into_bytes()
            .expect("reading response bytes")
            .to_vec();
        let body = String::from_utf8(body_bytes).expect("reading response bytes as string");
        let unescaped_body = from_str::<String>(&body).expect("reading unsecaped body");
        let feed: FeedDTO = from_str(&unescaped_body).expect("parsing feed body");
        assert!(feed.blogposts.len() == 0);
    }

    #[actix_web::test]
    async fn test_invalid_blogpost_bad_avatar() {
        let db_url = env::var("DB_URL")
            .expect("missing DB_URL in env");
        let connection_pool = establish_connection_pool(db_url)
            .expect("making a connection pool");

        let mut conn = connection_pool.get().expect("getting connection");

        let _ = web::block(move || { delete_all_posts(&mut conn) })
            .await
            .expect("deleting all posts");

        let app = test::init_service(
            App::new()
            .app_data(Data::new(connection_pool.clone()))
            .service(create_blogpost)
            .service(get_feed)
        ).await;

        let dto = CreateBlogPostDTO {
            text: "Hello!".to_string(),
            username: "admin".to_string(),
            avatar: Some("https://img.freepik.com/premium-psd/color-wing-png-isolated-transparent-background_1034016-9965.jpg".to_string()),
        };
        let dto_str = to_string(&dto).expect("turning dto to json string");

        let mut image_file = File::open(format!("images/placeholder_avatar"))
            .expect("opening placeholder avatar");

        let form = web::block(move || {create_multipart(dto_str, &mut image_file)})
            .await.unwrap();

        let req = test::TestRequest::post()
            .set_payload(form)
            .insert_header(("Content-Type", "multipart/form-data; boundary=my_boundary"))
            .uri("/api/v1/blogpost")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());

        // check if the created post is present
        let req = test::TestRequest::get()
            .uri("/api/v1/blogpost?page=1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = resp
            .into_body()
            .try_into_bytes()
            .expect("reading response bytes")
            .to_vec();
        let body = String::from_utf8(body_bytes).expect("reading response bytes as string");
        let unescaped_body = from_str::<String>(&body).expect("reading unsecaped body");
        let feed: FeedDTO = from_str(&unescaped_body).expect("parsing feed body");
        assert!(feed.blogposts.len() == 0);
    }
}

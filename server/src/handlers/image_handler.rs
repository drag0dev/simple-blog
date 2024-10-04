use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use log::{log, Level};

use crate::service::image_service;


/// serves the image based on the provided uuid in the path
#[get("/image/{uuid}")]
pub async fn get_image(uuid: web::Path<String>) -> impl Responder {
    let uuid = uuid.into_inner();

    let filename;
    if uuid == "placeholder_avatar" || uuid == "placeholder_image" { filename = uuid }
    else {
        let uuid = Uuid::try_parse(&uuid);
        if let Err(_) = uuid { return HttpResponse::BadRequest().finish(); }
        let uuid = uuid.unwrap();
        filename = uuid.to_string();
    }

    let image_res = image_service::get_image(filename)
        .await;

    if let Err(e) = image_res {
        log!(Level::Error, "Error opening image: {}", crate::unroll_anyhow_result(e));
        return HttpResponse::InternalServerError().finish()
    }

    let stream = image_res.unwrap();
    if stream.is_none() { return HttpResponse::BadRequest().finish(); }
    let stream = stream.unwrap(); // safe to unwrap because its always present

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .streaming(stream)
}

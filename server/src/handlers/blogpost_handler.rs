use std::collections::HashMap;
use std::time::Duration;
use actix_multipart::{Field, Multipart};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use futures_util::TryStreamExt;
use serde_json::to_string;
use tokio::time::timeout;
use crate::models::{FeedDTO, MAX_TEXT_SIZE, MAX_USERNAME_SIZE};
use crate::service::blogpost_service::get_blogposts;
use crate::service::image_service::{delete_image, save_image};
use crate::{models::CreateBlogPostDTO, service::blogpost_service};
use crate::db::DBPool;
use log::{log, Level};

/// helper function that remove the files if they have been saved
pub async fn clear_files(avatar: Option<String>, image: Option<String>) {
    if let Some(uuid) = avatar {
        let res = delete_image(uuid).await;
        if let Err(e) = res { log!(Level::Error, "Error removing an avatar: {}", crate::unroll_anyhow_result(e)); }
    }
    if let Some(uuid) = image {
        let res = delete_image(uuid).await;
        if let Err(e) = res { log!(Level::Error, "Error removing an image: {}", crate::unroll_anyhow_result(e)); }
    }
}

const CHUNK_TIMEOUT: Duration = Duration::from_secs(1);
const MAX_CHUNKS: u32 = 20;
/// used when draining the left over data before early return
pub async fn drain_data(payload: &mut Multipart, field: &mut Field) {
    while let Ok(Some(_bytes)) = field.try_next().await {}

    let mut chunk_count = 0;
    while chunk_count < MAX_CHUNKS {
        match timeout(CHUNK_TIMEOUT, payload.try_next()).await {
            Ok(Ok(Some(mut field))) => {
                while let Ok(Some(_bytes)) = field.try_next().await {}
            }
            _ => break
        }
        chunk_count += 1;
    }
}

#[post("/blogpost")]
async fn create_blogpost(mut payload: Multipart, pool: web::Data<DBPool>) -> impl Responder {
    let mut data_payload: Option<CreateBlogPostDTO> = None;
    let mut avatar_uuid: Option<String> = None;
    let mut post_image_uuid: Option<String> = None;

    // read the incoming data
    //
    // data can arrive in any order, therefore every time an error is
    // encountered already saved images have to be deleted
    //
    // force closing connection on every early return while there is still data to read,
    // otherwise connection will hang indefinitely
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        if content_disposition.is_none() {
            clear_files(avatar_uuid, post_image_uuid).await;
            drain_data(&mut payload, &mut field).await;
            return HttpResponse::BadRequest().force_close().finish();
        }
        let content_disposition = content_disposition.unwrap();

        let field_name = content_disposition.get_name();
        if field_name.is_none() {
            clear_files(avatar_uuid, post_image_uuid).await;
            drain_data(&mut payload, &mut field).await;
            return HttpResponse::BadRequest().force_close().finish();
        }
        let field_name = field_name.unwrap();

        match field_name {
            "data" => {
                let bytes = field.try_next().await;
                if let Err(e) = bytes {
                    log!(Level::Error, "Error reading data from the body: {}", e);
                    clear_files(avatar_uuid, post_image_uuid).await;
                    drain_data(&mut payload, &mut field).await;
                    return HttpResponse::InternalServerError().force_close().finish();
                }
                let bytes = bytes.unwrap();
                if bytes.is_none() {
                    clear_files(avatar_uuid, post_image_uuid).await;
                    drain_data(&mut payload, &mut field).await;
                    return HttpResponse::BadRequest().force_close().finish();
                }
                let bytes = bytes.unwrap();

                let deser_data = serde_json::from_slice(&bytes);
                if deser_data.is_err() {
                    clear_files(avatar_uuid, post_image_uuid).await;
                    drain_data(&mut payload, &mut field).await;
                    return HttpResponse::BadRequest().force_close().finish();
                }
                let deser_data: CreateBlogPostDTO = deser_data.unwrap();

                if deser_data.text.len() > MAX_TEXT_SIZE || deser_data.username.len() > MAX_USERNAME_SIZE {
                    clear_files(avatar_uuid, post_image_uuid).await;
                    drain_data(&mut payload, &mut field).await;
                    return HttpResponse::PayloadTooLarge().force_close().finish();
                }

                data_payload = Some(deser_data);
            }

            "avatar" => {
                let avatar_result = save_image(&mut field).await;
                if let Err(e) = avatar_result {
                    log!(Level::Error, "Error saving an avatar: {}", crate::unroll_anyhow_result(e));
                    clear_files(avatar_uuid, post_image_uuid).await;
                    drain_data(&mut payload, &mut field).await;
                    return HttpResponse::InternalServerError().force_close().finish();
                } else {
                    let (avatar_result, too_large, is_png) = avatar_result.unwrap();
                    if !too_large && is_png { avatar_uuid = Some(avatar_result); }
                    else if !is_png {
                        clear_files(avatar_uuid, post_image_uuid).await;
                        drain_data(&mut payload, &mut field).await;
                        return HttpResponse::BadRequest().force_close().finish();
                    }
                    else {
                        clear_files(avatar_uuid, post_image_uuid).await;
                        drain_data(&mut payload, &mut field).await;
                        return HttpResponse::PayloadTooLarge().force_close().finish();
                    }
                }
            }

            "image" => {
                let image_result = save_image(&mut field).await;
                if let Err(e) = image_result {
                    log!(Level::Error, "Error saving an image: {}", crate::unroll_anyhow_result(e));
                    clear_files(avatar_uuid, post_image_uuid).await;
                    drain_data(&mut payload, &mut field).await;
                    return HttpResponse::InternalServerError().force_close().finish();
                } else {
                    let (image_result, too_large, is_png) = image_result.unwrap();
                    if !too_large && is_png { post_image_uuid = Some(image_result); }
                    else if !is_png {
                        clear_files(avatar_uuid, post_image_uuid).await;
                        drain_data(&mut payload, &mut field).await;
                        return HttpResponse::BadRequest().force_close().finish();
                    }
                    else {
                        clear_files(avatar_uuid, post_image_uuid).await;
                        drain_data(&mut payload, &mut field).await;
                        return HttpResponse::PayloadTooLarge().force_close().finish();
                    }
                }
            }

            _ => return HttpResponse::BadRequest().force_close().finish()
        }
    }


    // post data cannot be missing
    if data_payload.is_none() {
        clear_files(avatar_uuid, post_image_uuid).await;
        return HttpResponse::BadRequest().finish();
    }

    let conn = pool.get();
    if let Err(e) = conn {
        log!(Level::Error, "Error getting a connection from pool: {e}");
        clear_files(avatar_uuid, post_image_uuid).await;
        return HttpResponse::InternalServerError().finish();
    }
    let mut conn = conn.unwrap();

    // persist the post data
    let avatar_uuid_clone = avatar_uuid.clone();
    let post_image_uuid_clone = post_image_uuid.clone();
    let res = web::block(move ||
        blogpost_service::create_blogpost(&mut conn, data_payload.unwrap(), avatar_uuid_clone, post_image_uuid_clone)
        ).await;
    if let Err(e) = res {
        log!(Level::Error, "Error saving blogpost into the db: {e}");
        clear_files(avatar_uuid, post_image_uuid).await;
        return HttpResponse::InternalServerError().finish();
    } else if let Err(e) = res.unwrap() {
        log!(Level::Error, "Error saving blogpost into the db: {}", crate::unroll_anyhow_result(e));
        clear_files(avatar_uuid, post_image_uuid).await;
        return HttpResponse::InternalServerError().finish();
    }


    HttpResponse::Created().finish()
}


#[get("/blogpost")]
async fn get_feed(req: HttpRequest, pool: web::Data<DBPool>) -> impl Responder {
    let params = web::Query::<HashMap<String, u32>>::from_query(req.query_string());
    if let Err(_) = params { return HttpResponse::BadRequest().finish(); }
    let params = params.unwrap();

    let page_str = params.get("page");
    if page_str.is_none() { return HttpResponse::BadRequest().finish(); }
    let page_num = page_str.unwrap();
    if *page_num < 1 { return HttpResponse::BadRequest().finish(); }

    let conn = pool.get();
    if let Err(e) = conn {
        log!(Level::Error, "Error getting a connection from pool: {e}");
        return HttpResponse::InternalServerError().finish();
    }
    let mut conn = conn.unwrap();

    let blogposts = get_blogposts(&mut conn, *page_num);
    if let Err(e) = blogposts {
        log!(Level::Error, "Error getting blogposts, page {}: {}", page_num, crate::unroll_anyhow_result(e));
        return HttpResponse::InternalServerError().finish();
    }
    let blogposts = blogposts.unwrap();
    let dto = FeedDTO::new(blogposts);

    let response_body = to_string(&dto);
    if let Err(e) = response_body {
        log!(Level::Error, "Error serializing feed dto: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(response_body.unwrap())
}

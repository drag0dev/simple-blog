use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures_util::TryStreamExt;
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

#[post("/blogpost")]
async fn create_blogpost(mut payload: Multipart, pool: web::Data<DBPool>) -> impl Responder {
    let mut data_payload: Option<CreateBlogPostDTO> = None;
    let mut avatar_uuid: Option<String> = None;
    let mut post_image_uuid: Option<String> = None;

    // read the incoming data
    // data can arrive in any order, therefore every time an error is
    // encountered already saved images have to be deleted
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        if content_disposition.is_none() {
            clear_files(avatar_uuid, post_image_uuid).await;
            return HttpResponse::BadRequest().finish();
        }
        let content_disposition = content_disposition.unwrap();

        let field_name = content_disposition.get_name();
        if field_name.is_none() {
            clear_files(avatar_uuid, post_image_uuid).await;
            return HttpResponse::BadRequest().finish();
        }
        let field_name = field_name.unwrap();

        match field_name {
            "data" => {
                let bytes = field.try_next().await;
                if let Err(e) = bytes {
                    log!(Level::Error, "Error reading data from the body: {}", e);
                    clear_files(avatar_uuid, post_image_uuid).await;
                    return HttpResponse::InternalServerError().finish();
                }
                let bytes = bytes.unwrap();
                if bytes.is_none() {
                    clear_files(avatar_uuid, post_image_uuid).await;
                    return HttpResponse::BadRequest().finish();
                }
                let bytes = bytes.unwrap();

                let deser_data = serde_json::from_slice(&bytes);
                if deser_data.is_err() {
                    clear_files(avatar_uuid, post_image_uuid).await;
                    return HttpResponse::BadRequest().finish();
                }

                data_payload = Some(deser_data.unwrap());
            }

            "avatar" => {
                let avatar_result = save_image(field).await;
                if let Err(e) = avatar_result {
                    log!(Level::Error, "Error saving an avatar: {}", crate::unroll_anyhow_result(e));
                    clear_files(avatar_uuid, post_image_uuid).await;
                    return HttpResponse::InternalServerError().finish();
                } else {
                    avatar_uuid = Some(avatar_result.unwrap());
                }
            }

            "image" => {
                let image_result = save_image(field).await;
                if let Err(e) = image_result {
                    log!(Level::Error, "Error saving an image: {}", crate::unroll_anyhow_result(e));
                    clear_files(avatar_uuid, post_image_uuid).await;
                    return HttpResponse::InternalServerError().finish();
                } else {
                    post_image_uuid = Some(image_result.unwrap());
                }
            }

            _ => return HttpResponse::BadRequest().finish()
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

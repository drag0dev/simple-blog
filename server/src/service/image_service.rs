use std::{fs::{remove_file, File}, io::Write};
use actix_web::web;
use anyhow::{anyhow, Context, Result};
use futures_util::TryStreamExt;
use uuid::Uuid;

const IMAGE_FILEPATH: &str = "./images";

/// saves image in the image folder
/// function returns the image uuid
pub async fn save_image(mut image: actix_multipart::Field) -> Result<String> {
    let image_id = Uuid::new_v4();
    let image_id = image_id.to_string();
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");

    let mut file = web::block(move || File::create(&filepath))
        .await
        .context("creating file")??;

    loop {
        let chunk = image.try_next().await;
        if let Err(e) = chunk { return Err(anyhow!(e.to_string())); }

        let chunk = chunk.unwrap();
        if chunk.is_none() { break; }

        file = web::block(move || file.write_all(&(chunk.unwrap())).map(|_| file))
            .await
            .context("writing image data")??;
    }

    Ok(image_id)
}

pub async fn delete_image(image_id: String) -> Result<()> {
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");
    web::block(move || remove_file(filepath))
        .await
        .context("deleting image: {image_id}")??;

    Ok(())
}

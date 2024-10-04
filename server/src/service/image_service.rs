use std::io::ErrorKind;
use anyhow::{anyhow, Context, Result};
use futures_util::TryStreamExt;
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::fs::{remove_file, File};
use crate::models::MAX_IMAGE_SIZE;

const IMAGE_FILEPATH: &str = "./images";

/// saves image in the image folder
/// function returns the image uuid and if image is larger than MAX_IMAGE_SIZE
/// if the image is larger than MAX_IMAGE_SIZE function will still return Ok, but the image has
/// been deleted
pub async fn save_image(mut image: actix_multipart::Field) -> Result<(String, bool)> {
    let image_id = Uuid::new_v4();
    let image_id = image_id.to_string();
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");

    let filepath_clone = filepath.clone();
    let mut file = File::create(&filepath_clone)
        .await
        .context("creating file")?;

    let mut chunk_size = 0;
    let mut chunk_too_large = false;

    loop {
        let chunk = image.try_next().await;
        if let Err(e) = chunk { return Err(anyhow!(e.to_string())); }

        let chunk = chunk.unwrap();
        if chunk.is_none() { break; }
        let chunk = chunk.unwrap();

        chunk_size += chunk.len();
        if chunk_size > MAX_IMAGE_SIZE {
            chunk_too_large = true;
            break;
        }

        file.write_all(&(chunk))
            .await
            .context("writing image data")?;
    }

    if chunk_too_large {
        remove_file(filepath)
            .await
            .context("deleting image that is too large: {image_id}")?;
        return Ok((image_id, true));
    }

    Ok((image_id, false))
}

pub async fn delete_image(image_id: String) -> Result<()> {
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");
    remove_file(filepath)
        .await
        .context("deleting image: {image_id}")?;

    Ok(())
}

/// if the file does not exist function returns Ok(None)
pub async fn get_image(image_id: String) -> Result<Option<ReaderStream<BufReader<tokio::fs::File>>>> {
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");

    let file = tokio::fs::File::open(&filepath)
        .await;

    if let Err(e) = file {
        if e.kind() == ErrorKind::NotFound {
            return Ok(None);
        } else {
            let e = anyhow!(e)
                .context(format!("opening image {}", image_id));
            return Err(e);
        }
    }
    let file = file.unwrap();

    let reader = BufReader::new(file);
    let stream = ReaderStream::new(reader);

    Ok(Some(stream))
}

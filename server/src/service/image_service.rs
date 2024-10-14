use std::io::ErrorKind;
use anyhow::{anyhow, Context, Result};
use futures_util::TryStreamExt;
use reqwest::StatusCode;
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::fs::{remove_file, File};
use crate::models::MAX_IMAGE_SIZE;

const IMAGE_FILEPATH: &str = "./images";
const PNG_MAGIC_BYTES: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

/// saves image in the image folder
/// function returns the image uuid, if image is larger than MAX_IMAGE_SIZE, and if image is PNG
/// in cases when the image is too large or not PNG, function still returns Ok
pub async fn save_image(image: &mut actix_multipart::Field) -> Result<(String, bool, bool)> {
    let image_id = Uuid::new_v4();
    let image_id = image_id.to_string();
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");

    let filepath_clone = filepath.clone();
    let mut file = File::create(&filepath_clone)
        .await
        .context("creating file")?;

    let mut chunk_size = 0;
    let mut chunk_too_large = false;
    let mut checked_format = false;

    loop {
        let chunk = image.try_next().await;
        if let Err(e) = chunk { return Err(anyhow!(e.to_string()).context("receiving image chunk")); }

        let chunk = chunk.unwrap();
        if chunk.is_none() { break; }
        let chunk = chunk.unwrap();

        chunk_size += chunk.len();
        if chunk_size > MAX_IMAGE_SIZE {
            chunk_too_large = true;
            break;
        }

        if !checked_format {
            let first_eight_bytes: Vec<u8> = chunk.iter().take(8).map(|b| *b).collect();
            if first_eight_bytes != PNG_MAGIC_BYTES {
                return Ok((image_id, false, false))
            }
            checked_format = true
        }

        file.write_all(&(chunk))
            .await
            .context("writing image data")?;
    }

    if chunk_too_large {
        remove_file(filepath)
            .await
            .context("deleting image that is too large: {image_id}")?;
        return Ok((image_id, true, true));
    }

    Ok((image_id, false, true))
}

pub async fn delete_image(image_id: String) -> Result<()> {
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");
    remove_file(filepath)
        .await
        .context("deleting image: {image_id}")?;

    Ok(())
}

/// function returns the image uuid if the image was successfully downloaded and saved
/// image uuid is None if the provided url is not an image, not a PNG,
/// or larger then MAX_IMAGE_SIZE
pub async fn download_avatar(image_url: &String) -> Result<Option<String>> {
    let mut response = reqwest::get(image_url)
        .await
        .context("downloading image from url {image_url}")?;

    if response.status() != StatusCode::OK { return Ok(None); }

    let total_size = response.content_length().unwrap_or(0);
    if total_size > MAX_IMAGE_SIZE as u64 { return Ok(None); }

    let image_id = Uuid::new_v4();
    let image_id = image_id.to_string();
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");
    let mut file = File::create(&filepath)
        .await
        .context("creating an image file")?;

    let mut downloaded_size = 0;
    let mut checked_format = false;

    while let Some(chunk) = response.chunk().await.context("reading image chunk")? {
        downloaded_size += chunk.len();
        if downloaded_size > MAX_IMAGE_SIZE { return Ok(None); }

        if !checked_format {
            let first_eight_bytes: Vec<u8> = chunk.iter().take(8).map(|b| *b).collect();
            if first_eight_bytes != PNG_MAGIC_BYTES {
                remove_file(&filepath)
                    .await
                    .context("deleting avatar because of wrong format")?;
                return Ok(None)
            }
            checked_format = true
        }

        let write_res = file.write_all(&chunk)
            .await
            .context("writing image chunk");

        if write_res.is_err() {
            remove_file(&filepath)
                .await
                .context("deleting avatar because of an error encountered while writing")?;
            return Err(write_res.err().unwrap());
        }
    }

    Ok(Some(image_id))
}

/// read the image from the local storage, if the file does not exist function returns Ok(None)
pub async fn get_image(image_id: String) -> Result<Option<ReaderStream<BufReader<File>>>> {
    let filepath = format!("{IMAGE_FILEPATH}/{image_id}");

    let file = tokio::fs::File::open(&filepath)
        .await;

    if let Err(e) = file {
        if e.kind() == ErrorKind::NotFound {
            return Ok(None);
        } else {
            let e = anyhow!(e)
                .context("opening image {image_id}");
            return Err(e);
        }
    }
    let file = file.unwrap();

    let reader = BufReader::new(file);
    let stream = ReaderStream::new(reader);

    Ok(Some(stream))
}

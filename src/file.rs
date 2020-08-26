use crate::{config::CONFIG, storage::STORAGE};
use log::debug;
use rocket::{get, response::Stream, tokio::io::AsyncRead};
use rusoto_s3::{GetObjectRequest, S3};
use std::path::PathBuf;

#[get("/files/<path..>")]
pub async fn file(path: PathBuf) -> Option<Stream<impl AsyncRead>> {
    let key = path.to_string_lossy().into_owned();

    match STORAGE
        .get_object(GetObjectRequest {
            bucket: CONFIG.s3.bucket.clone(),
            key: key.clone(),
            ..Default::default()
        })
        .await
    {
        Ok(result) => {
            if let Some(body) = result.body {
                return Some(Stream::chunked(body.into_async_read(), 16384));
            }
        }
        Err(err) => {
            debug!("could not read file {}: {}", key, err);
        }
    }

    None
}

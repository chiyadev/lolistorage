use crate::{
    config::{S3Config, CONFIG},
    header::RangeHeader,
};
use log::debug;
use once_cell::sync::Lazy;
use rusoto_core::{credential::StaticProvider, HttpClient, HttpConfig};
use rusoto_s3::{
    GetObjectOutput, GetObjectRequest, ListObjectsOutput, ListObjectsRequest, S3Client, S3,
};

pub const STORAGE_BUFFER_SIZE: usize = 1024 * 1024;

pub static STORAGE: Lazy<S3Client> = Lazy::new(|| {
    let S3Config {
        access_key,
        secret_key,
        region,
        ..
    } = CONFIG.s3.clone();

    let mut http_config = HttpConfig::new();
    http_config.read_buf_size(STORAGE_BUFFER_SIZE);

    S3Client::new_with(
        HttpClient::new_with_config(http_config).expect("could not create http client for s3"),
        StaticProvider::new_minimal(access_key, secret_key),
        region,
    )
});

pub async fn get_file(key: &str, range: Option<RangeHeader>) -> Option<GetObjectOutput> {
    let key = key.trim_start_matches('/');

    match STORAGE
        .get_object(GetObjectRequest {
            bucket: CONFIG.s3.bucket.clone(),
            key: key.into(),
            range: range.map(|r| r.to_header()),
            ..Default::default()
        })
        .await
    {
        Ok(result) => Some(result),
        Err(err) => {
            debug!("could not read file {}: {}", key, err);
            None
        }
    }
}

const LIST_SIZE: i64 = 200;

pub async fn list_dir(key: &str, marker: Option<String>) -> Option<ListObjectsOutput> {
    let key = key.trim_start_matches('/');

    let prefix = if key.len() == 0 {
        None
    } else {
        Some(format!("{}/", key))
    };

    match STORAGE
        .list_objects(ListObjectsRequest {
            bucket: CONFIG.s3.bucket.clone(),
            prefix,
            max_keys: Some(LIST_SIZE),
            delimiter: Some("/".into()),
            marker,
            ..Default::default()
        })
        .await
    {
        Ok(result) => Some(result),
        Err(err) => {
            debug!("could not list directory {}: {}", key, err);
            None
        }
    }
}

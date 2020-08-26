use crate::config::{S3Config, CONFIG};
use async_trait::async_trait;
use log::debug;
use once_cell::sync::Lazy;
use rusoto_core::{
    credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials},
    HttpClient,
};
use rusoto_s3::{
    GetObjectOutput, GetObjectRequest, ListObjectsOutput, ListObjectsRequest, S3Client, S3,
};

pub static STORAGE: Lazy<S3Client> = Lazy::new(|| {
    let S3Config {
        credentials,
        region,
        ..
    } = CONFIG.s3.clone();

    S3Client::new_with(
        HttpClient::new().expect("could not create http client for s3"),
        AwsCredentialsWrapper { credentials },
        region,
    )
});

struct AwsCredentialsWrapper {
    credentials: AwsCredentials,
}

#[async_trait]
impl ProvideAwsCredentials for AwsCredentialsWrapper {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        Ok(self.credentials.clone())
    }
}

pub async fn get_file(key: &str) -> Option<GetObjectOutput> {
    match STORAGE
        .get_object(GetObjectRequest {
            bucket: CONFIG.s3.bucket.clone(),
            key: key.into(),
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

use crate::config::{S3Config, CONFIG};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use rusoto_core::{
    credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials},
    HttpClient,
};
use rusoto_s3::S3Client;

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

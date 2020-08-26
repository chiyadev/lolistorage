use rocket::{http::Status, response::Responder, Request, Response};
use std::{error, fmt};

#[derive(Debug)]
pub enum LoliError {
    RusotoS3(rusoto_s3::S3Error),
    // Unknown,
}

impl error::Error for LoliError {}

impl From<rusoto_s3::S3Error> for LoliError {
    fn from(err: rusoto_s3::S3Error) -> LoliError {
        LoliError::RusotoS3(err)
    }
}

impl fmt::Display for LoliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoliError::RusotoS3(err) => write!(
                f,
                "{} ({})",
                err.message.clone().unwrap_or(String::new()),
                err.code.clone().unwrap_or(String::new())
            ),
            // LoliError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for LoliError {
    fn respond_to(self, request: &'r Request<'_>) -> Result<Response<'o>, Status> {
        Status::InternalServerError.respond_to(request)
    }
}

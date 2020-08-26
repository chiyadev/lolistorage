use crate::{
    header::RangeHeader,
    storage::{get_file, STORAGE_BUFFER_SIZE},
};
use rocket::{
    get,
    http::{ContentType, Header, Status},
    response::Responder,
    Request, Response,
};
use rusoto_s3::GetObjectOutput;
use std::{borrow::Cow, path::PathBuf};

#[get("/files/<path..>")]
pub async fn file(path: PathBuf, range: Option<RangeHeader>) -> Option<FileResponse> {
    get_file(path.to_string_lossy().as_ref(), range)
        .await
        .map(|x| FileResponse {
            path,
            result: Some(x),
        })
}

pub struct FileResponse {
    path: PathBuf,
    result: Option<GetObjectOutput>,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for FileResponse {
    fn respond_to(mut self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        if let Some(result) = self.result.take() {
            if let Some(stream) = result.body {
                let mut response = Response::<'o>::build();
                response.status(Status::Ok);

                response.header(Header::new(
                    "Content-Disposition",
                    result.content_disposition.map_or_else(
                        || {
                            format!(
                                "inline;filename={}",
                                self.path
                                    .file_name()
                                    .map(|s| s.to_string_lossy())
                                    .unwrap_or(Cow::Borrowed("unknown"))
                            )
                        },
                        |s| s,
                    ),
                ));

                if let Some(language) = result.content_language {
                    response.header(Header::new("Content-Language", language));
                }

                if let Some(length) = result.content_length {
                    response.header(Header::new("Content-Length", length.to_string()));
                }

                if let Some(range) = result.content_range {
                    response
                        .status(Status::PartialContent)
                        .header(Header::new("Content-Range", range));
                }

                response.header(result.content_type.map_or_else(
                    || ContentType::Binary.into(),
                    |s| Header::new("Content-Type", s.clone()),
                ));

                if let Some(e_tag) = result.e_tag {
                    response.header(Header::new("ETag", e_tag));
                }

                if let Some(modified) = result.last_modified {
                    response.header(Header::new("Last-Modified", modified));
                }

                if let Some(meta) = result.metadata {
                    for (key, value) in meta.iter() {
                        response.header(Header::new(key.clone(), value.clone()));
                    }
                }

                // body stream
                return response
                    .chunked_body(stream.into_async_read(), STORAGE_BUFFER_SIZE)
                    .ok();
            }
        }

        Err(Status::InternalServerError)
    }
}

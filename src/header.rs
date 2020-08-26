use async_trait::async_trait;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct RangeHeader {
    start: Option<i64>,
    end: Option<i64>,
}

impl RangeHeader {
    pub fn to_header(&self) -> String {
        format!(
            "bytes={}-{}",
            self.start.map_or(String::new(), |v| v.to_string()),
            self.end.map_or(String::new(), |v| v.to_string())
        )
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for RangeHeader {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        if let Some(range) = request.headers().get("Range").next() {
            if !range.contains(',') {
                let mut parts = range.splitn(2, '=');

                if let Some(unit) = parts.next() {
                    if unit == "bytes" {
                        if let Some(range) = parts.next() {
                            let mut parts = range.splitn(2, '-');

                            fn parse(s: Option<&str>) -> Result<Option<i64>, ()> {
                                if let Some(s) = s {
                                    if s.len() == 0 {
                                        return Ok(None);
                                    } else if let Ok(value) = s.parse() {
                                        return Ok(Some(value));
                                    }
                                }

                                Err(())
                            }

                            if let Ok(start) = parse(parts.next()) {
                                if let Ok(end) = parse(parts.next()) {
                                    return Outcome::Success(RangeHeader { start, end });
                                }
                            }
                        }
                    }
                }
            }

            Outcome::Failure((Status::BadRequest, ()))
        } else {
            Outcome::Forward(())
        }
    }
}

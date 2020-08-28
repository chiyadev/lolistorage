use crate::{
    api::{api, List},
    config::{Configuration, CONFIG},
    storage::get_file,
};
use async_trait::async_trait;
use pulldown_cmark::{html::push_html, Parser};
use rocket::{
    get,
    request::{FromRequest, Outcome},
    response::Redirect,
    tokio::io::AsyncReadExt,
    uri, Request,
};
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
struct ViewContext {
    title: String,
    config: &'static Configuration,
    list: List,
    path_parts: Vec<PathPart>,
    index_page: Option<IndexPage>,
}

#[derive(Serialize)]
struct PathPart {
    name: String,
    full_name: String,
}
#[derive(Serialize)]
struct IndexPage {
    name: String,
    full_name: String,
    content: Option<String>,
    content_raw: Option<String>,
}

#[get("/view?<key>")]
pub async fn dir_index(
    key: Option<String>,
    trailing_slash: TrailingSlashGuard,
) -> Result<Template, Redirect> {
    dir(PathBuf::new(), key, trailing_slash).await
}

#[get("/view/<path..>?<key>")]
pub async fn dir(
    path: PathBuf,
    key: Option<String>,
    trailing_slash: TrailingSlashGuard,
) -> Result<Template, Redirect> {
    // directory paths must have trailing slashes for relative links to work
    if !trailing_slash.0 {
        let path = path.to_string_lossy();

        return Err(if path.len() == 0 {
            Redirect::to("/view/")
        } else {
            Redirect::to(format!("/view/{}/", path))
        });
    }

    let list = api(path.clone(), key).await.into_inner();

    // list isn't valid, so we should be looking at a file instead
    if !list.valid {
        return Err(Redirect::to(uri!(crate::file::file: path)));
    }

    let title = path
        .file_name()
        .map_or(String::new(), |s| s.to_string_lossy().into_owned());

    let path_parts = {
        let mut parts = Vec::new();
        let mut joined = String::new();

        for part in path.iter() {
            let name = part.to_string_lossy().into_owned();

            if parts.len() != 0 {
                joined.push('/');
            }
            joined.push_str(&name);

            parts.push(PathPart {
                name,
                full_name: joined.clone(),
            })
        }

        parts
    };

    let index_page = {
        let mut page = None;

        let matches = list.files.iter().find(|f| {
            let name = f.name.to_lowercase();

            name.starts_with("index.") || name.starts_with("readme.")
        });

        for file in matches {
            let name = file.name.clone();
            let full_name = file.full_name.clone();

            let content = match get_file(&full_name, None).await.and_then(|r| r.body) {
                Some(result) => {
                    let mut buffer = String::new();

                    match result.into_async_read().read_to_string(&mut buffer).await {
                        Ok(_) => buffer,
                        Err(_) => continue,
                    }
                }
                None => continue,
            };

            let ext: &str = &name[name.rfind('.').unwrap_or(0)..].to_lowercase();

            page = match ext {
                ".html" => Some(IndexPage {
                    name,
                    full_name,
                    content: None,
                    content_raw: Some(content),
                }),

                ".md" => {
                    let parser = Parser::new_ext(&content, pulldown_cmark::Options::all());

                    let mut converted = String::new();

                    push_html(&mut converted, parser);

                    Some(IndexPage {
                        name,
                        full_name,
                        content: None,
                        content_raw: Some(converted),
                    })
                }

                _ => Some(IndexPage {
                    name,
                    full_name,
                    content: Some(content),
                    content_raw: None,
                }),
            };
            break;
        }

        page
    };

    Ok(Template::render(
        "view",
        &ViewContext {
            title,
            config: &CONFIG,
            list,
            path_parts,
            index_page,
        },
    ))
}

pub struct TrailingSlashGuard(bool);

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for TrailingSlashGuard {
    type Error = Redirect;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Outcome::Success(TrailingSlashGuard(request.uri().path().ends_with('/')))
    }
}

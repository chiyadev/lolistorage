use crate::{
    api::{api, List},
    config::{Configuration, CONFIG},
    storage::get_file,
};
use pulldown_cmark::{html::push_html, Parser};
use rocket::{get, response::Redirect, tokio::io::AsyncReadExt, uri};
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

#[get("/view?<begin>")]
pub async fn dir_index(begin: Option<String>) -> Result<Template, Redirect> {
    dir(PathBuf::new(), begin).await
}

#[get("/view/<path..>?<begin>")]
pub async fn dir(path: PathBuf, begin: Option<String>) -> Result<Template, Redirect> {
    let list = api(path.clone(), begin).await.into_inner();

    // if list is empty, it's probably a file
    if list.files.len() == 0 && list.directories.len() == 0 {
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

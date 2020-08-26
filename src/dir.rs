use crate::{
    api::{api, List},
    config::{Configuration, CONFIG},
};
use rocket::get;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
struct ViewContext {
    title: String,
    config: &'static Configuration,
    list: List,
    path_parts: Vec<PathPart>,
}

#[derive(Serialize)]
struct PathPart {
    name: String,
    full_name: String,
}

#[get("/view?<begin>")]
pub async fn dir_index(begin: Option<String>) -> Template {
    dir(PathBuf::new(), begin).await
}

#[get("/view/<path..>?<begin>")]
pub async fn dir(path: PathBuf, begin: Option<String>) -> Template {
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

    let list = api(path, begin).await.into_inner();

    Template::render(
        "view",
        &ViewContext {
            title,
            config: &CONFIG,
            list,
            path_parts,
        },
    )
}

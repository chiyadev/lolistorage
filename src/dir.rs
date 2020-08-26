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

    let list = api(path, begin).await.into_inner();

    Template::render(
        "view",
        &ViewContext {
            title,
            config: &CONFIG,
            list,
        },
    )
}

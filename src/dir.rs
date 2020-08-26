use crate::api::api;
use rocket::get;
use rocket_contrib::templates::Template;
use std::path::PathBuf;

#[get("/?<begin>")]
pub async fn dir_index(begin: Option<String>) -> Template {
    dir(PathBuf::new(), begin).await
}

#[get("/view/<path..>?<begin>")]
pub async fn dir(path: PathBuf, begin: Option<String>) -> Template {
    let list = api(path, begin).await.into_inner();

    Template::render("index", list)
}

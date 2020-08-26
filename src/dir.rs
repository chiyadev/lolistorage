use rocket::get;
use rocket_contrib::templates::Template;
use std::path::PathBuf;

#[get("/view/<path..>")]
pub async fn dir(path: PathBuf) -> Template {
    Template::render("index", ())
}

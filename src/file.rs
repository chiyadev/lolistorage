use rocket::get;
use rocket_contrib::json::Json;
use std::path::PathBuf;

#[get("/files/<path..>")]
pub async fn file(path: PathBuf) -> Json<()> {
    Json(())
}

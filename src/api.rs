use rocket::get;
use rocket_contrib::json::Json;
use std::path::PathBuf;

#[get("/api/<path..>")]
pub async fn api(path: PathBuf) -> Json<()> {
    Json(())
}

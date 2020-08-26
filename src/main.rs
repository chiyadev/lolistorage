#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, ignite, launch, routes, Rocket};
use rocket_contrib::templates::Template;
use std::path::PathBuf;

mod api;
mod storage;
mod config;
mod dir;
mod error;
mod file;

#[launch]
fn rocket() -> Rocket {
    ignite()
        .mount("/", routes![index, dir::dir, file::file, api::api])
        .attach(Template::fairing())
}

#[get("/")]
async fn index() -> Template {
    dir::dir(PathBuf::new()).await
}

#![feature(proc_macro_hygiene, decl_macro)]

use crate::config::CONFIG;
use rocket::{get, ignite, launch, routes, Rocket};
use rocket_contrib::templates::Template;
use std::path::PathBuf;

mod api;
mod config;
mod dir;
mod error;
mod file;
mod storage;

#[launch]
fn rocket() -> Rocket {
    env_logger::init_from_env(env_logger::Env::default().filter_or(
        "LOLI_LOG",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "warn"
        },
    ));

    // trigger initial config load
    let _ = CONFIG;

    ignite()
        .mount("/", routes![index, dir::dir, file::file, api::api])
        .attach(Template::fairing())
}

#[get("/")]
async fn index() -> Template {
    dir::dir(PathBuf::new()).await
}

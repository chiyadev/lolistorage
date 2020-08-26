#![feature(proc_macro_hygiene, decl_macro)]

use crate::{
    config::CONFIG,
    handlebars::{icon_helper, if_exists_helper, if_not_null_helper},
};
use rocket::{get, ignite, launch, response::Redirect, routes, uri, Rocket};
use rocket_contrib::templates::Template;

mod api;
mod config;
mod dir;
mod error;
mod file;
mod handlebars;
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
        .mount(
            "/",
            routes![
                index,
                file::file,
                dir::dir,
                dir::dir_index,
                api::api,
                api::api_index
            ],
        )
        .attach(Template::custom(|engine| {
            engine
                .handlebars
                .register_helper("if_exists", Box::new(if_exists_helper));

            engine
                .handlebars
                .register_helper("if_not_null", Box::new(if_not_null_helper));

            engine
                .handlebars
                .register_helper("icon", Box::new(icon_helper));
        }))
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to(uri!(dir::dir_index: _))
}

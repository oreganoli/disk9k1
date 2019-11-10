#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_map_or_else)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate serde;

use rocket_contrib::serve;

use instance::Instance;
use prelude::*;

mod file;
mod instance;
mod prelude;
pub mod schema;
mod upload;
mod user;
mod util;

#[get("/")]
fn index(instance: LockState, tera: TeraState) -> Page {
    let inst = instance.read().unwrap();
    let data = inst.ins_repo.get().unwrap();
    let mut ctx = Context::new();
    ctx.insert("instance", &data);
    tera.html("PAGE_index.html", &ctx)
}

#[get("/login")]
fn login_or_register(tera: TeraState) -> Page {
    tera.html("PAGE_login_or_register.html", &Context::new())
}

fn main() {
    #[cfg(debug_assertions)] // Only load env vars from .env in dev builds
    dotenv::dotenv().ok();
    let instance = Instance::new();
    let tera = Tera::new("templates/**/*").expect("Expected a template directory.");
    let renderer = Renderer(tera);
    rocket::ignite()
        .manage(renderer)
        .manage(std::sync::RwLock::new(instance))
        .mount(
            "/",
            routes![
                index,
                login_or_register //                file::file_info,
                                  //                file::get_file,
                                  //                file::get_file_named,
                                  //                upload::upload
            ],
        )
        .mount(
            "/static",
            serve::StaticFiles::new("static/", serve::Options::None),
        )
        .launch();
}

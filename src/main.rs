#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_map_or_else)]
#[macro_use]
extern crate rocket;

use std::collections::BTreeMap;

use rocket_contrib::serve;

use instance::Instance;
use prelude::*;

mod file;
mod instance;
mod prelude;
mod upload;
mod util;

#[get("/")]
fn index(instance: LockState, tera: TeraState) -> Page {
    let inst = instance.read().unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", &inst.name);
    ctx.insert("description", &inst.description);
    ctx.insert("size_limit", &inst.size_limit);
    tera.html("index.html", &ctx)
}

fn main() {
    let instance = Instance {
        name: "Disk9001".to_owned(),
        description: "A pomf.se and Google Drive clone. WIP.".to_owned(),
        size_limit: 8388608,
        files: BTreeMap::new(),
    };
    let tera = Tera::new("templates/**/*").expect("Expected a template directory.");
    let renderer = Renderer(tera);
    rocket::ignite()
        .manage(renderer)
        .manage(std::sync::RwLock::new(instance))
        .mount(
            "/",
            routes![
                index,
                file::file_info,
                file::get_file,
                file::get_file_named,
                upload::upload
            ],
        )
        .mount(
            "/static",
            serve::StaticFiles::new("static/", serve::Options::None),
        )
        .launch();
}

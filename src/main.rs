#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::io::Read;

use rocket_contrib::serve;

use instance::Instance;
use prelude::*;

mod instance;
mod prelude;
mod upload;

#[get("/")]
fn index(instance: LockState, tera: TeraState) -> Html<String> {
    let inst = instance.read().unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", &inst.name);
    ctx.insert("description", &inst.description);
    ctx.insert("size_limit", &inst.size_limit);
    Html(tera.render("index.html", &ctx).unwrap())
}

fn main() {
    let instance = Instance {
        name: "Disk9001".to_owned(),
        description: "A pomf.se and Google Drive clone. WIP.".to_owned(),
        size_limit: 8388608
    };
    let tera = Tera::new("templates/**/*").expect("Expected a template directory.");
    rocket::ignite()
        .manage(tera)
        .manage(std::sync::RwLock::new(instance))
        .mount("/", routes![index, upload::upload])
        .mount("/static", serve::StaticFiles::new("static/", serve::Options::None))
        .launch();
}

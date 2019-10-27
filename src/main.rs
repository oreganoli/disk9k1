#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::io::Read;

use rocket::Data;
use rocket_contrib::serve;

use instance::Instance;
use prelude::*;

mod instance;
mod prelude;

#[post("/upload", data = "<data>")]
fn upload(data: Data) -> Html<String> {
    let mut buffer = String::new();
    data.open().read_to_string(&mut buffer).unwrap();
    Html(format!("<b>{}</b>", buffer))
}

#[get("/")]
fn index(tera: State<Tera>, instance: State<Instance>) -> Html<String> {
    let ctx = Context::from_serialize(instance.inner()).unwrap();
    Html(tera.render("index.html", &ctx).unwrap())
}

fn main() {
    let instance = Instance {
        name: "Disk9001".to_owned(),
        description: "A pomf.se and Google Drive clone. WIP.".to_owned(),
    };
    let tera = Tera::new("templates/**/*").expect("Expected a template directory.");
    rocket::ignite()
        .manage(tera)
        .manage(instance)
        .mount("/", routes![index, upload])
        .mount("/static", serve::StaticFiles::new("static/", serve::Options::None))
        .launch();
}

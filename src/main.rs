#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod prelude;
use prelude::*;
use rocket_contrib::serve;

#[get("/")]
fn index() -> String {
    "Hello world!".to_owned()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static", serve::StaticFiles::new("static/", serve::Options::None))
        .launch();
}

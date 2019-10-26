#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    "Hello world!".to_owned()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

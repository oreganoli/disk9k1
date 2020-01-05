#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_map_or_else)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
extern crate serde;

use rocket_contrib::serve;

use prelude::*;
use util::lock::Lock;

mod app;
mod content;
pub mod error;
mod instance;
mod prelude;
mod user;
pub mod util;

lazy_static! {
    pub static ref INDEX: String = std::fs::read_to_string("html/index.html")
        .expect("There should be an index.html file in /html");
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(&INDEX)
}

fn main() -> AppResult<()> {
    #[cfg(debug_assertions)] // Only load env vars from .env in dev builds
    dotenv::dotenv().ok();
    let pool = util::pool::create_pool();
    let app = Lock(RwLock::new(App::new(pool)?));
    rocket::ignite()
        .manage(app)
        .mount("/", routes![index])
        .mount("/", routes![instance::get, instance::put])
        .mount(
            "/",
            routes![
                user::me,
                user::get,
                user::get_all,
                user::post,
                user::put_password,
                user::put_username,
                user::delete,
                user::login,
                user::logout
            ],
        )
        .mount(
            "/",
            routes![
                content::dirs::get,
                content::dirs::get_top,
                content::dirs::post,
                content::dirs::put_name,
                content::dirs::put_parent,
                content::dirs::delete,
            ],
        )
        .mount(
            "/static",
            serve::StaticFiles::new("static/", serve::Options::None),
        )
        .mount(
            "/js",
            serve::StaticFiles::new("js/dist/", serve::Options::None),
        )
        .launch();
    Ok(())
}

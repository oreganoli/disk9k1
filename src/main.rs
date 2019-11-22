#![feature(proc_macro_hygiene, decl_macro)]
#![feature(result_map_or_else)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
extern crate serde;

use rocket_contrib::serve;

use prelude::*;
use util::lock::Lock;

mod directory;
mod error;
//mod file;
mod instance;
mod prelude;
pub mod schema;
//mod upload;
mod user;
pub mod util;

lazy_static! {
    pub static ref INDEX: String = std::fs::read_to_string("html/index.html")
        .expect("There should be an index.html file in /html");
}

fn main() {
    #[cfg(debug_assertions)] // Only load env vars from .env in dev builds
    dotenv::dotenv().ok();
    let app = Lock(RwLock::new(Instance::default()));
    rocket::ignite()
        .manage(app)
        .mount(
            "/",
            routes![
                instance::index,
                instance::instance,
                user::auth::authenticate,
                user::auth::logout,
                user::delete::delete_account,
                user::info::get_user,
                user::info::get_me,
                user::register::register,
                instance::settings::modify_instance,
                //                user::settings::change_password,
                //                user::settings::change_email,
                //                file::file_info,
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

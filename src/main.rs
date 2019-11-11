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
                instance::index,
                instance::settings::modify_instance,
                instance::settings::panel,
                instance::users::users,
                user::auth::authenticate,
                user::auth::login,
                user::auth::logout,
                user::delete::delete_account,
                user::info::get_user,
                user::info::get_me,
                user::register::register,
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

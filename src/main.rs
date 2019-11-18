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
use util::renderer::Renderer;

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
    /// A globally accessible `Instance` behind a `Lock`
    pub static ref INSTANCE: Lock<Instance> = Lock(RwLock::new(Instance::default()));
    /// The global template renderer.
    pub static ref TERA: Lock<Renderer> = Lock(RwLock::new(Renderer::default()));
}

fn main() {
    #[cfg(debug_assertions)] // Only load env vars from .env in dev builds
    dotenv::dotenv().ok();
    rocket::ignite()
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
                user::delete::del_acc_confirm,
                user::delete::delete_account,
                user::info::get_user,
                user::info::get_me,
                user::register::register,
                user::settings::settings,
                user::settings::change_password,
                user::settings::change_email,
                user::settings::change_username,
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

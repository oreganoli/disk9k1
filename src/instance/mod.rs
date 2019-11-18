use std::collections::BTreeMap;

use repo::{InstanceRepo, InstanceRepository};
use schema::instance;

use crate::prelude::*;
use crate::user::repo::{UserRepo, UserRepository};
use crate::user::NewUser;

mod repo;
pub mod settings;
pub mod users;

/// The "god struct" that holds Disk9001's state.
pub struct Instance {
    /// The repo for the global `InstanceData`.
    pub ins_repo: Box<dyn InstanceRepository + Sync + Send>,
    /// The repo for user data.
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
}

impl Default for Instance {
    fn default() -> Self {
        let mut ins_repo = InstanceRepo::new();
        if ins_repo.get().unwrap().is_none() {
            eprintln!("No instance data was found, assuming first run and populating with default values.");
            ins_repo.set(InstanceData::default()).unwrap();
        }
        let mut user_repo = UserRepo::new();
        if user_repo.read_all().unwrap().is_empty() {
            eprintln!("No accounts were found. Generating admin account from env variables...");
            user_repo.create(NewUser::generate_admin()).unwrap();
        }
        Self {
            ins_repo: Box::new(ins_repo),
            user_repo: Box::new(user_repo),
        }
    }
}

/// The data global to the Disk9001 instance.
#[derive(Clone, Debug, FromForm, Serialize, Queryable, Insertable)]
#[table_name = "instance"]
pub struct InstanceData {
    /// The instance's name.
    pub name: String,
    /// The welcome text shown on the index page.
    pub description: String,
    /// The maximum size of files users can upload.
    pub size_limit: i64,
}

impl std::default::Default for InstanceData {
    fn default() -> Self {
        Self {
            name: "Disk9001".to_owned(),
            description: "A pomf.se and Google Drive clone. WIP.".to_owned(),
            size_limit: 8_388_608,
        }
    }
}

#[get("/")]
pub fn index(mut cookies: Cookies) -> Result<Page, Error> {
    let inst = instance_read();
    let mut ctx = Context::new();
    ctx.insert("instance", &inst.ins_repo.get()?);
    let user = inst.user_from_cookies(&mut cookies);
    if let Some(u) = user {
        ctx.insert("user", &u.to_info())
    };
    Ok(render("PAGE_index.html", &ctx))
}

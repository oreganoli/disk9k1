use std::collections::BTreeMap;
use std::ops::Deref;

use rocket_contrib::json::Json;

use repo::{InstanceRepo, InstanceRepository};
use schema::instance;

use crate::directory::repo::{DirectoryRepo, DirectoryRepository};
use crate::prelude::*;
use crate::user::repo::{UserRepo, UserRepository};
use crate::user::NewUser;

mod repo;
pub mod settings;

/// The "god struct" that holds Disk9001's state.
pub struct Instance {
    /// The repo for the global `InstanceData`.
    pub ins_repo: Box<dyn InstanceRepository + Sync + Send>,
    /// The repo for user data.
    pub user_repo: Box<dyn UserRepository + Sync + Send>,

    pub dir_repo: Box<dyn DirectoryRepository + Sync + Send>,
}

impl Default for Instance {
    fn default() -> Self {
        let mut ins_repo = InstanceRepo::new();
        if ins_repo
            .get()
            .expect("Can't get the instance repo's cache.")
            .is_none()
        {
            eprintln!("No instance data was found, assuming first run and populating with default values.");
            ins_repo.set(InstanceData::default()).unwrap();
        }
        let mut user_repo = UserRepo::new();
        if user_repo
            .read_all()
            .expect("Can't read all the users")
            .is_empty()
        {
            eprintln!("No accounts were found. Generating admin account from env variables...");
            user_repo
                .create(NewUser::generate_admin())
                .expect("Could not create an admin account");
        }
        let dir_repo = DirectoryRepo::new();
        Self {
            ins_repo: Box::new(ins_repo),
            user_repo: Box::new(user_repo),
            dir_repo: Box::new(dir_repo),
        }
    }
}

/// The data global to the Disk9001 instance.
#[derive(Clone, Debug, Deserialize, Serialize, Queryable, Insertable)]
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
pub fn index() -> Html<&'static str> {
    Html(crate::INDEX.deref())
}

#[get("/instance")]
pub fn instance(app: AppState) -> Json<InstanceData> {
    let inst = app.read();
    Json(inst.ins_repo.get().unwrap().unwrap())
}

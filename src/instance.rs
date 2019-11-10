use std::collections::BTreeMap;

use repo::{InstanceRepo, InstanceRepository};
use schema::instance;

use crate::file::File;
use crate::prelude::*;

mod repo;

/// The "god struct" that holds Disk9001's state.
pub struct Instance {
    /// The repo for the global `InstanceData`.
    pub ins_repo: Box<dyn InstanceRepository + Sync + Send>,
    pub files: BTreeMap<u32, File>,
}

impl Instance {
    pub fn new() -> Self {
        let mut repo = InstanceRepo::new();
        if repo.get().unwrap().is_none() {
            eprintln!("No instance data was found, assuming first run and populating with default values.");
            repo.set(InstanceData::default()).unwrap();
        }
        Self {
            ins_repo: Box::new(repo),
            files: BTreeMap::new(),
        }
    }
}

/// The data global to the Disk9001 instance.
#[derive(Clone, Debug, Serialize, Queryable, Insertable)]
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
            size_limit: 8388608,
        }
    }
}

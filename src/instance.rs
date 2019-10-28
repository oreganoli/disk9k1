use std::collections::BTreeMap;

use crate::file::File;
use crate::prelude::*;

pub struct Instance {
    pub name: String,
    pub description: String,
    pub size_limit: usize,
    pub files: BTreeMap<u32, File>
}
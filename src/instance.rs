use crate::prelude::*;

#[derive(Serialize)]
pub struct Instance {
    pub name: String,
    pub description: String,
    pub size_limit: usize,
}
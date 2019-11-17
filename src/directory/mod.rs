use schema::directories;

use crate::prelude::*;

mod repo;

#[derive(Queryable)]
pub struct Directory {
    id: i32,
    owner: i32,
    name: String,
    parent: Option<i32>,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "directories"]
pub struct NewDirectory {
    owner: i32,
    name: String,
    parent: Option<i32>,
}

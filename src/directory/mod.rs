use schema::directories;

use crate::prelude::*;

mod repo;

#[derive(Associations, Queryable, Identifiable)]
#[belongs_to(Directory, foreign_key = "id")]
#[belongs_to(User, foreign_key = "owner")]
#[table_name = "directories"]
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

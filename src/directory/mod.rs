use schema::directories;

use crate::prelude::*;

pub mod read;
pub mod repo;

#[derive(Associations, Clone, Queryable, Serialize, Identifiable)]
#[belongs_to(Directory, foreign_key = "parent")]
#[belongs_to(User, foreign_key = "owner")]
#[table_name = "directories"]
pub struct Directory {
    id: i32,
    owner: i32,
    name: String,
    parent: Option<i32>,
    created: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "directories"]
pub struct NewDirectory {
    owner: i32,
    name: String,
    parent: Option<i32>,
}

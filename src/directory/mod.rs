use crate::prelude::*;

#[derive(Queryable)]
pub struct Directory {
    id: i32,
    owner: i32,
    name: String,
    parent: Option<i32>,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}

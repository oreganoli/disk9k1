use crate::prelude::*;

use super::ContentRepo;

pub struct Directory {
    id: i32,
    name: String,
    owner: i32,
    parent: Option<i32>,
}

impl ContentRepo {
    pub fn read(&self, id: i32, conn: &mut Conn) -> AppResult<Option<Directory>> {
        let dir = conn
            .query(include_str!("sql/dirs/read.sql"), &[&id])?
            .first()
            .map(|row| Directory {
                id: row.get(0),
                name: row.get(1),
                owner: row.get(2),
                parent: row.get(3),
            });
        Ok(dir)
    }
    pub fn read_children(&self, id: i32, conn: &mut Conn) -> AppResult<Vec<Directory>> {
        let dirs = conn
            .query(include_str!("sql/dirs/read_children.sql"), &[&id])?
            .iter()
            .map(|row| Directory {
                id: row.get(0),
                name: row.get(1),
                owner: row.get(2),
                parent: row.get(3),
            })
            .collect::<Vec<_>>();
        Ok(dirs)
    }
    pub fn read_top(&self, user_id: i32, conn: &mut Conn) -> AppResult<Vec<Directory>> {
        let dirs = conn
            .query(include_str!("sql/dirs/read_top_user.sql"), &[&user_id])?
            .iter()
            .map(|row| Directory {
                id: row.get(0),
                name: row.get(1),
                owner: row.get(2),
                parent: row.get(3),
            })
            .collect::<Vec<_>>();
        Ok(dirs)
    }
}

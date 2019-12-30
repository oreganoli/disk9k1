use crate::prelude::*;

use super::ContentRepo;

pub struct Directory {
    id: i32,
    name: String,
    owner: i32,
    parent: Option<i32>,
}

pub struct NewDir {
    name: String,
    owner: i32,
    parent: Option<i32>,
}

pub(crate) enum DirError {
    NamingConflict,
    CyclicParenthood,
}

impl ContentRepo {
    pub fn create(&self, new: NewDir, conn: &mut Conn) -> AppResult<()> {
        // check for naming conflicts
        let naming_ok = conn
            .query(
                include_str!("sql/dirs/creation_legal.sql"),
                &[&new.name, &new.owner, &new.parent],
            )?
            .first()
            .map_or(false, |row| row.get(0));
        //check for ownership conflict
        let ownership_ok = if new.parent.is_none() {
            true
        } else {
            conn.query(
                include_str!("sql/dirs/owner_same.sql"),
                &[&new.owner, &new.parent],
            )?
            .first()
            .map_or(false, |row| row.get(0))
        };
        if !(ownership_ok) {
            return Err(AuthError::NotAllowed.into());
        }
        if !(naming_ok) {
            return Err(DirError::NamingConflict.into());
        }
        conn.execute(
            include_str!("sql/dirs/create.sql"),
            &[&new.name, &new.owner, &new.parent],
        )?;
        Ok(())
    }
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

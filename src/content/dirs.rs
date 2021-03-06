use crate::content::file::File;
use crate::prelude::*;

use super::DirectoryRepo;

#[derive(Serialize)]
pub struct DirView {
    id: i32,
    name: String,
    owner: i32,
    parent: Option<i32>,
    children: Vec<Directory>,
    files: Vec<File>, // TODO add child files
}

#[derive(Serialize)]
pub struct Directory {
    id: i32,
    name: String,
    owner: i32,
    parent: Option<i32>,
}

fn get_files(owner: i32, id: Option<i32>, conn: &mut Conn) -> AppResult<Vec<File>> {
    let files = conn.query(
        "SELECT id, filename, hash, owner, public, directory FROM files WHERE owner = $1 AND directory IS NOT DISTINCT FROM $2;",
        &[&owner, &id],
    )?.iter().map(|row| {
        File {
            id: row.get(0),
            filename: row.get(1),
            hash: row.get(2),
            owner: row.get(3),
            public: row.get(4),
            directory: row.get(5),
        }
    }).collect::<Vec<_>>();
    Ok(files)
}

impl Directory {
    pub fn into_view(self, repo: &DirectoryRepo, conn: &mut Conn) -> AppResult<DirView> {
        let children = repo.read_children(self.id, conn)?;
        let files = get_files(self.owner, Some(self.id), conn)?;
        Ok(DirView {
            id: self.id,
            name: self.name,
            owner: self.owner,
            parent: self.parent,
            children,
            files,
        })
    }
}
#[derive(Deserialize)]
pub struct NewDir {
    name: String,
    owner: i32,
    parent: Option<i32>,
}

pub(crate) enum DirError {
    NameInvalid,
    NamingConflict,
    CyclicParenthood,
    Nonexistent,
    NonexistentParent,
}

impl DirectoryRepo {
    pub fn create(&self, new: NewDir, conn: &mut Conn) -> AppResult<()> {
        if !self.dir_regex.is_match(&new.name) {
            return Err(DirError::NameInvalid.into());
        }
        // check for naming conflicts
        let naming_ok = conn
            .query(
                include_str!("sql/dirs/naming_conflicts.sql"),
                &[&new.name, &new.owner, &new.parent],
            )?
            .first()
            .map_or(false, |row| row.get(0));
        let parent_exists = match new.parent {
            None => true,
            Some(parent) => conn
                .query(include_str!("sql/dirs/exists_parent.sql"), &[&parent])?
                .first()
                .map_or(false, |row| row.get(0)),
        };
        //check for ownership conflict
        let ownership_ok = match new.parent {
            None => true,
            _ => conn
                .query(
                    include_str!("sql/dirs/owner_same.sql"),
                    &[&new.owner, &new.parent],
                )?
                .first()
                .map_or(false, |row| row.get(0)),
        };
        if !(parent_exists) {
            return Err(DirError::NonexistentParent.into());
        }
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
    pub fn update_name(&self, id: i32, name: &str, conn: &mut Conn) -> AppResult<()> {
        if !self.dir_regex.is_match(name) {
            return Err(DirError::NameInvalid.into());
        }
        let dir = match self.read(id, conn)? {
            None => return Err(DirError::Nonexistent.into()),
            Some(d) => d,
        };
        let validity = conn
            .query(
                include_str!("sql/dirs/naming_conflicts.sql"),
                &[&name, &dir.owner, &dir.parent],
            )?
            .first()
            .map_or(false, |row| row.get(0));
        if !validity {
            Err(DirError::NamingConflict.into())
        } else {
            conn.execute(include_str!("sql/dirs/update_name.sql"), &[&id, &name])?;
            Ok(())
        }
    }
    pub fn update_parent(&self, id: i32, new: Option<i32>, conn: &mut Conn) -> AppResult<()> {
        let moved = match self.read(id, conn)? {
            None => return Err(DirError::Nonexistent.into()),
            Some(d) => d,
        };
        let target_parent: Option<Directory>;
        if let Some(new_dir) = new {
            if id == new_dir {
                return Err(DirError::CyclicParenthood.into());
            }
            target_parent = self.read(new_dir, conn)?;
            if target_parent.is_none() {
                return Err(DirError::NonexistentParent.into());
            }
            if moved.owner != target_parent.unwrap().owner {
                return Err(AuthError::NotAllowed.into());
            }
        }
        let validity = conn
            .query(
                include_str!("sql/dirs/naming_conflicts.sql"),
                &[&moved.name, &moved.owner, &new],
            )?
            .first()
            .map_or(false, |row| row.get(0));
        if !validity {
            Err(DirError::NamingConflict.into())
        } else {
            conn.execute(include_str!("sql/dirs/update_parent.sql"), &[&id, &new])?;
            Ok(())
        }
    }
    pub fn delete(&self, id: i32, conn: &mut Conn) -> AppResult<()> {
        self.read(id, conn)?.map_or(Ok(()), |_| {
            conn.execute(include_str!("sql/dirs/delete.sql"), &[&id])?;
            Ok(())
        })
    }
}

#[get("/drive")]
pub fn get_top(app: AppState, mut cookies: Cookies) -> AppResult<Json<DirView>> {
    let app = app.read();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let children = app.dirs.read_top(user.id, conn)?;
    let view = DirView {
        id: 0,
        name: "/".to_owned(),
        owner: user.id,
        parent: None,
        children,
        files: get_files(user.id, None, conn)?,
    };
    Ok(Json(view))
}

#[get("/drive/<id>")]
pub fn get(app: AppState, mut cookies: Cookies, id: i32) -> AppResult<Option<Json<DirView>>> {
    let app = app.read();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let dir = match app.dirs.read(id, conn)? {
        None => return Ok(None),
        Some(d) => d,
    };
    if user.id != dir.owner {
        return Err(AuthError::NotAllowed.into());
    }
    let view = dir.into_view(&app.dirs, conn)?;
    Ok(Some(Json(view)))
}

#[post("/drive", data = "<new>")]
pub fn post(app: AppState, mut cookies: Cookies, new: Json<NewDir>) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    if user.id != new.owner {
        return Err(AuthError::NotAllowed.into());
    }
    app.dirs.create(new.into_inner(), conn)
}

#[derive(Deserialize)]
pub struct DirRename {
    id: i32,
    name: String,
}

#[put("/rename_dir", data = "<ren>")]
pub fn put_name(app: AppState, mut cookies: Cookies, ren: Json<DirRename>) -> AppResult<()> {
    let app = app.write();
    if !app.dirs.dir_regex.is_match(&ren.name) {
        return Err(DirError::NameInvalid.into());
    }
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let dir = match app.dirs.read(ren.id, conn)? {
        Some(d) => d,
        None => return Err(DirError::Nonexistent.into()),
    };
    if dir.owner != user.id {
        return Err(AuthError::NotAllowed.into());
    }
    app.dirs.update_name(ren.id, &ren.name, conn)
}

#[derive(Deserialize)]
pub struct DirMove {
    id: i32,
    new: Option<i32>,
}

#[put("/move_dir", data = "<dir_move>")]
pub fn put_parent(app: AppState, mut cookies: Cookies, dir_move: Json<DirMove>) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    let moved = match app.dirs.read(dir_move.id, conn)? {
        Some(d) => d,
        None => return Err(DirError::Nonexistent.into()),
    };
    if user.id != moved.owner {
        return Err(AuthError::NotAllowed.into());
    }
    app.dirs.update_parent(dir_move.id, dir_move.new, conn)
}

#[derive(Deserialize)]
pub struct RmDir {
    id: i32,
}

#[delete("/drive", data = "<rmdir>")]
pub fn delete(app: AppState, mut cookies: Cookies, rmdir: Json<RmDir>) -> AppResult<()> {
    let app = app.write();
    let conn = &mut app.pool.get()?;
    let user = app.user.user_from_cookies(&mut cookies, conn)?;
    match app.dirs.read(rmdir.id, conn)? {
        None => Ok(()), // idempotent!
        Some(dir) => {
            if user.id != dir.owner {
                Err(AuthError::NotAllowed.into())
            } else {
                app.dirs.delete(rmdir.id, conn)
            }
        }
    }
}

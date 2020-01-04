use regex::Regex;

use crate::prelude::*;

pub mod data;
pub mod dirs;

pub struct DirectoryRepo {
    dir_regex: Regex,
}

pub struct DataRepo {}

impl DataRepo {
    pub fn init(&self, conn: &mut Conn) -> AppResult<()> {
        conn.execute(include_str!("sql/data/init.sql"), &[])?;
        Ok(())
    }
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {};
        rep.init(conn)?;
        Ok(rep)
    }
}

impl DirectoryRepo {
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {
            dir_regex: Regex::new(r#"(?m)(^\.?[^.\r\n\t\\/:"|?*<>]+[^\r\n\t\\/:"|?*<>]*$)"#)
                .unwrap(),
        };
        rep.init(conn)?;
        Ok(rep)
    }
    pub fn init(&self, conn: &mut Conn) -> AppResult<()> {
        conn.execute(include_str!("sql/dirs/init.sql"), &[])?;
        Ok(())
    }
}

pub struct FileRepo {
    filename_regex: Regex,
}

impl FileRepo {
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {
            filename_regex: Regex::new(r#"(?m)(^\.?[^.\r\n\t\\/:"|?*<>]+[^\r\n\t\\/:"|?*<>]*$)"#)
                .unwrap(),
        };
        rep.init(conn)?;
        Ok(rep)
    }
    pub fn init(&self, conn: &mut Conn) -> AppResult<()> {
        conn.execute(include_str!("sql/file/init.sql"), &[])?;
        Ok(())
    }
}

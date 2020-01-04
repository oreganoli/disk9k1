use regex::Regex;

use crate::prelude::*;

pub struct DirectoryRepo {
    dir_regex: Regex,
}

pub mod dirs;

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
        conn.execute(include_str!("sql/data/init.sql"), &[])?;
        conn.execute(include_str!("sql/dirs/init.sql"), &[])?;
        conn.execute(include_str!("sql/file/init.sql"), &[])?;
        Ok(())
    }
}

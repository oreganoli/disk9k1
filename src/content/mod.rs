use regex::Regex;

use crate::prelude::*;

pub struct ContentRepo {
    folder_regex: Regex,
}

pub mod dirs;

impl ContentRepo {
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {
            folder_regex: Regex::new(r#"(?m)(^\.?[^.\r\n\t\\/:"|?*<>]+[^\r\n\t\\/:"|?*<>]*$)"#)
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

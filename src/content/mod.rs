use crate::prelude::*;

pub struct ContentRepo {}

impl ContentRepo {
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {};
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

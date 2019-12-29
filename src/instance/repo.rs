use crate::prelude::*;

use super::Instance;

pub struct InstanceRepo {}

impl InstanceRepo {
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {};
        rep.init(conn)?;
        Ok(rep)
    }

    fn init(&self, conn: &mut Conn) -> AppResult<()> {
        conn.execute(include_str!("sql/init.sql"), &[])?;
        conn.execute(include_str!("sql/populate.sql"), &[])?;
        Ok(())
    }

    pub fn update(&self, inst: Instance, conn: &mut Conn) -> AppResult<()> {
        conn.execute(
            include_str!("sql/update.sql"),
            &[&inst.name, &inst.description, &(inst.size_limit as i64)],
        )?;
        Ok(())
    }

    pub fn read(&self, conn: &mut Conn) -> AppResult<Instance> {
        let row = conn.query_one(include_str!("sql/read.sql"), &[])?;
        Ok(Instance {
            name: row.get(0),
            description: row.get(1),
            size_limit: row.get::<usize, i64>(2) as usize,
            version: env!("CARGO_PKG_VERSION").to_owned(),
        })
    }
}

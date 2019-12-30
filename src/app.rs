use crate::prelude::*;

pub struct App {
    pub(crate) instance: InstanceRepo,
    pub user: UserRepo,
    pub(crate) pool: Pool,
}

impl App {
    pub fn new(pool: Pool) -> AppResult<Self> {
        let mut conn = pool.get()?;
        Ok(Self {
            instance: InstanceRepo::new(&mut conn)?,
            user: UserRepo::new(&mut conn)?,
            pool,
        })
    }
}

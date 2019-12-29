use crate::prelude::*;

pub struct App {
    pub(crate) instance: InstanceRepo,
    pub(crate) pool: Pool,
}

impl App {
    pub fn new(pool: Pool) -> AppResult<Self> {
        Ok(Self {
            instance: InstanceRepo::new(&mut pool.get()?)?,
            pool,
        })
    }
}

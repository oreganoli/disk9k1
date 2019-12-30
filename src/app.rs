use crate::prelude::*;

pub struct App {
    pub(crate) instance: InstanceRepo,
    pub user: UserRepo,
    pub content: ContentRepo,
    pub(crate) pool: Pool,
}

impl App {
    pub fn new(pool: Pool) -> AppResult<Self> {
        let conn = &mut pool.get()?;
        let instance = InstanceRepo::new(conn)?;
        let user = UserRepo::new(conn)?;
        let content = ContentRepo::new(conn)?;
        Ok(Self {
            instance,
            user,
            content,
            pool,
        })
    }
}

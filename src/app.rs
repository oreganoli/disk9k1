use crate::prelude::*;

pub struct App {
    pub data: DataRepo,
    pub files: FileRepo,
    pub(crate) instance: InstanceRepo,
    pub user: UserRepo,
    pub dirs: DirectoryRepo,
    pub(crate) pool: Pool,
}

impl App {
    pub fn new(pool: Pool) -> AppResult<Self> {
        let conn = &mut pool.get()?;
        let instance = InstanceRepo::new(conn)?;
        let user = UserRepo::new(conn)?;
        let dirs = DirectoryRepo::new(conn)?;
        let data = DataRepo::new(conn)?;
        let files = FileRepo::new(conn)?;
        conn.execute("DROP FUNCTION IF EXISTS prune;", &[])?;
        conn.execute(include_str!("content/sql/prune.sql"), &[])?;
        conn.execute("DROP TRIGGER IF EXISTS prune_trigger ON files;", &[])?;
        conn.execute(include_str!("content/sql/prune_trigger.sql"), &[])?;
        Ok(Self {
            data,
            files,
            instance,
            user,
            dirs,
            pool,
        })
    }
}

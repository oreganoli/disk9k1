use super::*;

pub trait DirectoryRepository {
    fn create(&mut self, new: NewDirectory) -> AppResult<i32>;
    fn read(&self, id: i32) -> AppResult<Option<Directory>>;
    fn read_children(&self, id: i32) -> AppResult<Vec<Directory>>;
    fn read_top_for_user(&self, user: &User) -> AppResult<Vec<Directory>>;
    fn update_name(&mut self, id: i32, name: String) -> AppResult<()>;
    fn update_parent(&mut self, id: i32, new_parent: i32) -> AppResult<()>;
    fn update_date(&mut self, id: i32) -> AppResult<()>;
    fn delete(&mut self, id: i32) -> AppResult<()>;
}

pub struct DirectoryRepo {
    pool: HandledPool,
}

impl DirectoryRepo {
    pub fn new() -> Self {
        Self {
            pool: HandledPool::default(),
        }
    }
}

impl DirectoryRepository for DirectoryRepo {
    fn create(&mut self, new: NewDirectory) -> AppResult<i32> {
        unimplemented!()
    }

    fn read(&self, id: i32) -> AppResult<Option<Directory>> {
        directories::table
            .find(id)
            .first(&self.pool.get())
            .optional()
            .map_err(|f| AppError::Db)
    }

    fn read_children(&self, id: i32) -> AppResult<Vec<Directory>> {
        self.read(id)?.map_or_else(
            || AppError::dir(DirectoryError::Nonexistent),
            |d| {
                Directory::belonging_to(&d)
                    .load::<Directory>(&self.pool.get())
                    .map_err(|_| AppError::Db)
            },
        )
    }

    fn read_top_for_user(&self, user: &User) -> AppResult<Vec<Directory>> {
        Directory::belonging_to(user)
            .load::<Directory>(&self.pool.get())
            .map_err(|_| AppError::Db)
    }

    fn update_name(&mut self, id: i32, name: String) -> AppResult<()> {
        unimplemented!()
    }

    fn update_parent(&mut self, id: i32, new_parent: i32) -> AppResult<()> {
        unimplemented!()
    }

    fn update_date(&mut self, id: i32) -> AppResult<()> {
        unimplemented!()
    }

    fn delete(&mut self, id: i32) -> AppResult<()> {
        unimplemented!()
    }
}

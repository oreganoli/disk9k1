use super::*;

pub trait DirectoryRepository {
    fn create(&mut self, new: NewDirectory) -> Result<i32, Error>;
    fn read(&self, id: i32) -> Result<Option<Directory>, Error>;
    fn read_children(&self, id: i32) -> Result<Vec<Directory>, Error>;
    fn update_name(&mut self, id: i32, name: String) -> Result<(), Error>;
    fn update_parent(&mut self, id: i32, new_parent: i32) -> Result<(), Error>;
    fn update_date(&mut self, id: i32) -> Result<(), Error>;
    fn delete(&mut self, id: i32) -> Result<(), Error>;
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

use super::*;

pub trait DirectoryRepository {
    fn create(&mut self, new: NewDirectory) -> Result<i32, Error>;
    fn read(&self, id: i32) -> Result<Option<Directory>, Error>;
    fn read_children(&self, id: i32) -> Result<Vec<Directory>, Error>;
    fn read_top_for_user(&self, user: &User) -> Result<Vec<Directory>, Error>;
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

impl DirectoryRepository for DirectoryRepo {
    fn create(&mut self, new: NewDirectory) -> Result<i32, Error> {
        unimplemented!()
    }

    fn read(&self, id: i32) -> Result<Option<Directory>, Error> {
        directories::table
            .find(id)
            .first(&self.pool.get())
            .optional()
            .map_err(|f| Error::Db)
    }

    fn read_children(&self, id: i32) -> Result<Vec<Directory>, Error> {
        self.read(id)?.map_or_else(
            || Error::dir(DirectoryError::Nonexistent),
            |d| {
                Directory::belonging_to(&d)
                    .load::<Directory>(&self.pool.get())
                    .map_err(|_| Error::Db)
            },
        )
    }

    fn read_top_for_user(&self, user: &User) -> Result<Vec<Directory>, Error> {
        Directory::belonging_to(user)
            .load::<Directory>(&self.pool.get())
            .map_err(|_| Error::Db)
    }

    fn update_name(&mut self, id: i32, name: String) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_parent(&mut self, id: i32, new_parent: i32) -> Result<(), Error> {
        unimplemented!()
    }

    fn update_date(&mut self, id: i32) -> Result<(), Error> {
        unimplemented!()
    }

    fn delete(&mut self, id: i32) -> Result<(), Error> {
        unimplemented!()
    }
}

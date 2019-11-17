use schema::users;

use crate::prelude::*;
use crate::user::NewUser;

use super::User;

/// The interface for `User` repositories.
pub trait UserRepository {
    fn create(&mut self, new_user: NewUser) -> Result<(), Error>;
    fn read_all(&self) -> Result<Vec<User>, Error>;
    fn read_by_id(&self, id: i32) -> Result<Option<User>, Error>;
    fn read_by_name(&self, name: String) -> Result<Option<User>, Error>;
    fn update_name(&mut self, id: i32, new_name: String) -> Result<(), Error>;
    fn update_email(&mut self, id: i32, new_email: String) -> Result<(), Error>;
    fn update_password(&mut self, id: i32, new_password: String) -> Result<(), Error>;
    fn update_quick_token(&mut self, id: i32, new_token: String) -> Result<(), Error>;
    fn delete(&mut self, id: i32) -> Result<(), Error>;
}

/// The concrete Diesel repo.
pub struct UserRepo {
    pool: HandledPool,
    /// A cache of values.
    cache: Option<Vec<User>>,
}

impl UserRepo {
    pub fn new() -> Self {
        let mut repo = Self {
            pool: HandledPool::new(),
            cache: None,
        };
        repo.update_cache().unwrap();
        repo
    }
    fn update_cache(&mut self) -> Result<(), Error> {
        let conn = self.pool.get();
        users::table.load(&conn).map_or_else(
            |_| Err(Error::Db),
            |u| {
                self.cache = Some(u);
                Ok(())
            },
        )
    }
}

impl UserRepository for UserRepo {
    /// Takes a user with an unhashed password and token.
    fn create(&mut self, new_user: NewUser) -> Result<(), Error> {
        let mut user = new_user;
        user.password = bcrypt::hash(user.password, BCRYPT_COST).unwrap();
        user.quick_token = bcrypt::hash(user.quick_token, BCRYPT_COST).unwrap();
        diesel::insert_into(users::table)
            .values(user)
            .execute(&self.pool.get())
            .map_or_else(
                |_| Err(Error::Db),
                |_| {
                    self.update_cache().unwrap();
                    Ok(())
                },
            )
    }
    fn read_all(&self) -> Result<Vec<User>, Error> {
        Ok(self.cache.clone().unwrap())
    }

    fn read_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        self.cache
            .as_ref()
            .expect("User cache should not be empty.")
            .iter()
            .find(|x| x.id == id)
            .map_or_else(|| Ok(None), |u| Ok(Some(u.clone())))
    }

    fn read_by_name(&self, name: String) -> Result<Option<User>, Error> {
        self.cache
            .as_ref()
            .expect("User cache should not be empty.")
            .iter()
            .find(|x| x.name == name)
            .map_or_else(|| Ok(None), |u| Ok(Some(u.clone())))
    }

    fn update_name(&mut self, id: i32, new_name: String) -> Result<(), Error> {
        let conn = self.pool.get();
        diesel::update(users::table.find(id))
            .set(users::name.eq(new_name))
            .execute(&conn)
            .map_or_else(
                |_| Err(Error::Db),
                |_| {
                    self.update_cache().unwrap();
                    Ok(())
                },
            )
    }
    fn update_email(&mut self, id: i32, new_email: String) -> Result<(), Error> {
        let conn = self.pool.get();
        diesel::update(users::table.find(id))
            .set(users::email.eq(new_email))
            .execute(&conn)
            .map_or_else(
                |_| Err(Error::Db),
                |_| {
                    self.update_cache().unwrap();
                    Ok(())
                },
            )
    }
    /// Takes an unhashed `password`.
    fn update_password(&mut self, id: i32, new_password: String) -> Result<(), Error> {
        let conn = self.pool.get();
        diesel::update(users::table.find(id))
            .set(users::password.eq(bcrypt::hash(new_password, BCRYPT_COST).unwrap()))
            .execute(&conn)
            .map_or_else(
                |_| Err(Error::Db),
                |_| {
                    self.update_cache().unwrap();
                    Ok(())
                },
            )
    }
    /// Takes an unhashed token.
    fn update_quick_token(&mut self, id: i32, new_token: String) -> Result<(), Error> {
        let conn = self.pool.get();
        diesel::update(users::table.find(id))
            .set(users::password.eq(bcrypt::hash(new_token, BCRYPT_COST).unwrap()))
            .execute(&conn)
            .map_or_else(
                |_| Err(Error::Db),
                |_| {
                    self.update_cache().unwrap();
                    Ok(())
                },
            )
    }
    fn delete(&mut self, id: i32) -> Result<(), Error> {
        let conn = self.pool.get();
        diesel::delete(users::table.find(id))
            .execute(&conn)
            .map_or_else(
                |_| Err(Error::Db),
                |_| {
                    self.update_cache().unwrap();
                    Ok(())
                },
            )
    }
}

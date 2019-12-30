use regex::Regex;

use crate::prelude::*;

use super::UserError;
use super::{NewUser, User};

pub struct UserRepo {
    name_regex: Regex,
    mail_regex: Regex,
}

impl UserRepo {
    pub fn new(conn: &mut Conn) -> AppResult<Self> {
        let rep = Self {
            name_regex: Regex::new(r"^[^<>`~!\\@#}/$%:;)(^{&*=|'+\s]+$").unwrap(),
            mail_regex: Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap(),
        };
        rep.init(conn)?;
        Ok(rep)
    }
    fn init(&self, conn: &mut Conn) -> AppResult<()> {
        conn.execute(include_str!("sql/init.sql"), &[])?;
        self.populate(conn)?;
        Ok(())
    }
    fn populate(&self, conn: &mut Conn) -> AppResult<()> {
        use std::env::var;
        if !conn
            .query(include_str!("sql/exists_admin.sql"), &[])?
            .first()
            .map_or(false, |row| row.get(0))
        {
            eprintln!("No admin account exists, creating a new one with default environment variable data...");
            let name = var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set, panicking.");
            let passwd = var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set, panicking.");
            let email = var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set, panicking.");
            let new = NewUser {
                name,
                email,
                password: passwd.clone(),
                pass_con: passwd,
                is_admin: true,
            };
            self.create(new, conn)?;
        }
        Ok(())
    }
    pub fn create(&self, user: NewUser, conn: &mut Conn) -> AppResult<()> {
        if user.password.is_empty() || user.password.chars().count() < 16 as usize {
            return Err(UserError::PasswordInvalid.into());
        }
        if !&user.password.eq(&user.pass_con) {
            return Err(UserError::PasswordsNotMatching.into());
        }
        if !self.name_regex.is_match(&user.name) {
            return Err(UserError::NameInvalid.into());
        }
        if !self.mail_regex.is_match(&user.email) {
            return Err(UserError::EmailInvalid.into());
        }
        if conn
            .query(include_str!("sql/exists_name.sql"), &[&user.name])?
            .first()
            .map_or(false, |row| row.get(0))
        {
            return Err(UserError::NameTaken.into());
        }
        if conn
            .query(include_str!("sql/exists_email.sql"), &[&user.email])?
            .first()
            .map_or(false, |row| row.get(0))
        {
            return Err(UserError::EmailTaken.into());
        }

        let hashed = bcrypt::hash(user.password, BCRYPT_COST)?;

        conn.execute(
            include_str!("sql/create.sql"),
            &[&user.name, &user.email, &hashed, &user.is_admin],
        )?;
        Ok(())
    }
    pub fn read(&self, id: i32, conn: &mut Conn) -> AppResult<Option<User>> {
        let user = conn
            .query(include_str!("sql/read_id.sql"), &[&id])?
            .iter()
            .next()
            .map(|row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
                is_admin: row.get(4),
            });
        Ok(user)
    }
    pub fn read_all(&self, conn: &mut Conn) -> AppResult<Vec<User>> {
        let users = conn
            .query(include_str!("sql/read.sql"), &[])?
            .iter()
            .map(|row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
                is_admin: row.get(4),
            })
            .collect::<Vec<_>>();
        Ok(users)
    }
    pub fn read_name(&self, name: &str, conn: &mut Conn) -> AppResult<Option<User>> {
        let user = conn
            .query(include_str!("sql/read_username.sql"), &[&name])?
            .first()
            .map(|row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
                is_admin: row.get(4),
            });
        Ok(user)
    }
    pub fn user_from_cookies(&self, cookies: &mut Cookies, conn: &mut Conn) -> AppResult<User> {
        use super::AuthError;
        let name = if let Some(nck) = cookies.get_private("username") {
            nck.value().to_owned()
        } else {
            return Err(AuthError::InvalidCredentials.into());
        };
        let pass = if let Some(pck) = cookies.get_private("password") {
            pck.value().to_owned()
        } else {
            return Err(AuthError::InvalidCredentials.into());
        };
        if let Some(user) = self.read_name(&name, conn)? {
            if !bcrypt::verify(pass, &user.password)? {
                Err(AuthError::InvalidCredentials.into())
            } else {
                Ok(user)
            }
        } else {
            Err(AuthError::InvalidCredentials.into())
        }
    }
}

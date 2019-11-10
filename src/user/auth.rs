use crate::prelude;

use super::{User, UserInfo};

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap()
    }
    pub fn verify_token(&self, token: &str) -> bool {
        bcrypt::verify(token, &self.quick_token).unwrap()
    }
}

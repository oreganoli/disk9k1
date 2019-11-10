use crate::prelude::*;

use super::{User, UserInfo};

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap()
    }
    pub fn verify_token(&self, token: &str) -> bool {
        bcrypt::verify(token, &self.quick_token).unwrap()
    }
}

impl Instance {
    pub fn user_from_cookies(&self, cookies: &mut Cookies) -> Option<User> {
        let username = match cookies.get_private("username") {
            Some(u) => u,
            _ => return None,
        };
        let password = match cookies.get_private("password") {
            Some(p) => p,
            _ => return None,
        };
        let user = self
            .user_repo
            .read_by_name(username.value().to_string())
            .unwrap();
        user.and_then(|u| {
            if u.verify_password(password.value()) {
                Some(u.clone())
            } else {
                None
            }
        })
    }
}

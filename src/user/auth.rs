use crate::prelude::*;

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

#[derive(FromForm)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[post("/authenticate", data = "<auth_req>")]
pub fn authenticate(
    instance: LockState,
    mut cookies: Cookies,
    auth_req: Form<AuthRequest>,
) -> Result<Redirect, Redirect> {
    let inst = instance.read().unwrap();
    let user = inst
        .user_repo
        .read_by_name(auth_req.username.clone())
        .unwrap();
    user.map_or_else(
        || Err(Redirect::to(uri!(crate::login_or_register))),
        |u| {
            if u.verify_password(&auth_req.password) {
                cookies.add_private(Cookie::new("username", auth_req.username.clone()));
                cookies.add_private(Cookie::new("password", auth_req.password.clone()));
                Ok(Redirect::to(uri!(crate::index)))
            } else {
                Err(Redirect::to(uri!(crate::login_or_register)))
            }
        },
    )
}

/// Self-explanatory.
/// This is a POST so web accelerators and other automated clients don't log the user out in an attempt to pre-fetch data.
#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("username"));
    cookies.remove_private(Cookie::named("password"));
    Redirect::to(uri!(crate::index))
}

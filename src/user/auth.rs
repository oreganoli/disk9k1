use std::convert::TryFrom;

use rocket::http::uri::Uri;

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
    pub login_redirect: String,
}

#[post("/authenticate", data = "<auth_req>")]
pub fn authenticate(
    instance: LockState,
    mut cookies: Cookies,
    auth_req: Form<AuthRequest>,
) -> Result<Redirect, Error> {
    let inst = instance.read().unwrap();
    let user = inst.user_repo.read_by_name(auth_req.username.clone())?;
    user.map_or_else(
        || Error::user_auth(AuthError::BadCredentials),
        |u| {
            if u.verify_password(&auth_req.password) {
                cookies.add_private(Cookie::new("username", auth_req.username.clone()));
                cookies.add_private(Cookie::new("password", auth_req.password.clone()));
                Ok(Redirect::to(
                    Uri::try_from(auth_req.login_redirect.clone())
                        .unwrap_or(Uri::try_from("/").unwrap()),
                ))
            } else {
                Error::user_auth(AuthError::BadCredentials)
            }
        },
    )
}

#[get("/login")]
pub fn login(tera: TeraState) -> Page {
    tera.html("PAGE_login.html", &Context::new())
}

/// Self-explanatory.
/// This is a POST so web accelerators and other automated clients don't log the user out in an attempt to pre-fetch data.
#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("username"));
    cookies.remove_private(Cookie::named("password"));
    Redirect::to(uri!(crate::instance::index))
}

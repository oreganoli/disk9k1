use std::convert::TryFrom;

use rocket::http::uri::Uri;

use crate::prelude::*;

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap()
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
                Some(u)
            } else {
                None
            }
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[post("/authenticate", data = "<auth_req>")]
pub fn authenticate(
    app: AppState,
    mut cookies: Cookies,
    auth_req: Json<AuthRequest>,
) -> AppResult<Json<bool>> {
    let inst = app.read();
    let user = inst.user_repo.read_by_name(auth_req.username.clone())?;
    user.map_or_else(
        || {
            eprintln!("auth failed");
            Ok(Json(false))
        },
        |u| {
            if u.verify_password(&auth_req.password) {
                cookies.add_private(Cookie::new("username", auth_req.username.clone()));
                cookies.add_private(Cookie::new("password", auth_req.password.clone()));
                eprintln!("Auth succeeded");
                Ok(Json(true))
            } else {
                eprintln!("Auth failed");
                Ok(Json(false))
            }
        },
    )
}
/// Self-explanatory.
/// This is a POST so web accelerators and other automated clients don't log the user out in an attempt to pre-fetch data.
#[post("/logout")]
pub fn logout(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("username"));
    cookies.remove_private(Cookie::named("password"));
}

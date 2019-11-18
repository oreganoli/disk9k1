use crate::prelude::*;

#[get("/user/<id>")]
pub fn get_user(mut cookies: Cookies, id: i32) -> Result<Option<()>, Error> {
    let inst = instance_read();
    unimplemented!()
}

#[get("/me")]
pub fn get_me(mut cookies: Cookies) -> Option<Redirect> {
    let inst = instance_read();
    match inst.user_from_cookies(&mut cookies) {
        Some(u) => Some(Redirect::to(format!("/user/{}", u.id))),
        None => None,
    }
}

use crate::prelude::*;

#[get("/user/<id>")]
pub fn get_user(mut cookies: Cookies, id: i32) -> Result<Option<Page>, Error> {
    let inst = instance_read();
    let mut ctx = Context::new();
    ctx.insert("instance", &inst.ins_repo.get()?);
    let user = inst.user_from_cookies(&mut cookies);
    match user {
        Some(u) => ctx.insert("user", &u.to_info()),
        None => (),
    }
    let requested_user = inst.user_repo.read_by_id(id)?;
    match requested_user {
        Some(u) => {
            ctx.insert("requested_user", &u.to_info());
            Ok(Some(render("PAGE_user.html", &ctx)))
        }
        None => Ok(None),
    }
}

#[get("/me")]
pub fn get_me(mut cookies: Cookies) -> Option<Redirect> {
    let inst = instance_read();
    match inst.user_from_cookies(&mut cookies) {
        Some(u) => Some(Redirect::to(format!("/user/{}", u.id))),
        None => None,
    }
}

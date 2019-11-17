use crate::prelude::*;

#[get("/users")]
pub fn users(mut cookies: Cookies) -> Result<Page, Error> {
    let inst = instance_read();
    let mut ctx = Context::new();
    ctx.insert("instance", &inst.ins_repo.get()?);
    let user = inst.user_from_cookies(&mut cookies);
    if let Some(u) = user {
        ctx.insert("user", &u.to_info())
    };
    let users = inst.user_repo.read_all().unwrap();
    let user_infos = users.iter().map(|u| u.to_info()).collect::<Vec<_>>();
    ctx.insert("users", &user_infos);
    Ok(render("PAGE_users.html", &ctx))
}

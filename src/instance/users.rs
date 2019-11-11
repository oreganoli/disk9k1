use crate::prelude::*;

#[get("/users")]
pub fn users(instance: LockState, tera: TeraState, mut cookies: Cookies) -> Page {
    let inst = instance.read().unwrap();
    let mut ctx = Context::new();
    ctx.insert("instance", &inst.ins_repo.get().unwrap());
    let user = inst.user_from_cookies(&mut cookies);
    match user {
        Some(u) => ctx.insert("user", &u.to_info()),
        None => (),
    };
    let users = inst.user_repo.read_all().unwrap();
    let user_infos = users.iter().map(|u| u.to_info()).collect::<Vec<_>>();
    ctx.insert("users", &user_infos);
    tera.html("PAGE_users.html", &ctx)
}

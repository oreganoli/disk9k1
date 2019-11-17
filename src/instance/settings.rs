use crate::prelude::*;

pub enum InstanceSettingsError {
    NameEmpty,
    NegativeSizeLimit,
    NotAllowed,
}

impl Instance {
    pub fn set_instance_data(
        &mut self,
        new_data: InstanceData,
        requester: &User,
    ) -> Result<(), InstanceSettingsError> {
        if new_data.name.is_empty() {
            return Err(InstanceSettingsError::NameEmpty);
        }
        if new_data.size_limit < 0 {
            return Err(InstanceSettingsError::NegativeSizeLimit);
        }
        if requester.is_admin() {
            self.ins_repo.set(new_data).unwrap();
            Ok(())
        } else {
            Err(InstanceSettingsError::NotAllowed)
        }
    }
}

#[post("/modify_instance", data = "<ins_req>")]
pub fn modify_instance(
    instance: LockState,
    mut cookies: Cookies,
    ins_req: Form<InstanceData>,
) -> Result<Redirect, Redirect> {
    let mut inst = instance.write().unwrap();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Err(Redirect::to(uri!(crate::user::auth::login))),
    };
    match inst.set_instance_data(ins_req.into_inner(), &user) {
        Ok(()) => Ok(Redirect::to(uri!(panel))),
        Err(InstanceSettingsError::NotAllowed) => Err(Redirect::to(uri!(crate::user::auth::login))),
        Err(_) => Err(Redirect::to(uri!(panel))),
    }
}

#[get("/panel")]
pub fn panel(instance: LockState, tera: TeraState, mut cookies: Cookies) -> Result<Page, Redirect> {
    let inst = instance.read().unwrap();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Err(Redirect::to(uri!(crate::user::auth::login))),
    };
    if user.to_info().is_admin {
        let mut ctx = Context::new();
        ctx.insert("instance", &inst.ins_repo.get().unwrap());
        ctx.insert("user", &user.to_info());
        Ok(tera.html("PAGE_panel.html", &ctx))
    } else {
        Err(Redirect::to(uri!(crate::user::auth::login)))
    }
}

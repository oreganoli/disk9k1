use crate::prelude::*;

impl Instance {
    pub fn set_instance_data(
        &mut self,
        new_data: InstanceData,
        requester: &User,
    ) -> Result<(), Error> {
        if new_data.name.is_empty() {
            return Error::instance(InstanceError::NameEmpty);
        }
        if new_data.size_limit < 0 {
            return Error::instance(InstanceError::NegativeSizeLimit);
        }
        if requester.is_admin() {
            self.ins_repo.set(new_data).unwrap();
            Ok(())
        } else {
            Error::user_auth(AuthError::Unauthorized("/panel".to_owned()))
        }
    }
}

#[post("/modify_instance", data = "<ins_req>")]
pub fn modify_instance(
    mut cookies: Cookies,
    ins_req: Form<InstanceData>,
) -> Result<Redirect, Error> {
    let mut inst = instance_write();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Error::user_auth(AuthError::Unauthenticated("panel".to_owned())),
    };
    inst.set_instance_data(ins_req.into_inner(), &user)
        .map(|_| Redirect::to("/panel"))
}

#[get("/panel")]
pub fn panel(mut cookies: Cookies) -> Result<Page, Error> {
    let inst = instance_read();
    let user = match inst.user_from_cookies(&mut cookies) {
        Some(u) => u,
        None => return Error::user_auth(AuthError::Unauthenticated("panel".to_owned())),
    };
    if user.to_info().is_admin {
        let mut ctx = Context::new();
        ctx.insert("instance", &inst.ins_repo.get()?);
        ctx.insert("user", &user.to_info());
        Ok(render("PAGE_panel.html", &ctx))
    } else {
        Error::user_auth(AuthError::Unauthorized("panel".to_owned()))
    }
}

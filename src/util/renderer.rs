use std::error::Error;

use crate::prelude::*;
use crate::TERA;

/// A newtype that wraps `Tera` so we can avoid writing the `Html` wrapper and `.unwrap()`ping the value over and over.
pub struct Renderer(pub Tera);

impl Default for Renderer {
    fn default() -> Self {
        Self(Tera::new("templates/**/*").expect("Expected a template directory."))
    }
}
impl Renderer {
    #[cfg(debug_assertions)]
    pub fn html(&mut self, name: &str, ctx: &Context) -> Page {
        let mut ctx = ctx.clone();
        self.0
            .full_reload()
            .expect("Could not reload the template directory.");
        Html(self.0.render(name, &ctx).unwrap_or_else(|f| {
            let string = format!("Error description: {}, kind: {:?}", f, f.source());
            ctx.insert("reason", &string);
            self.0.render("PAGE_template_error.html", &ctx).unwrap()
        }))
    }
    #[cfg(not(debug_assertions))]
    pub fn html(&self, name: &str, ctx: &Context) -> Page {
        let mut ctx = ctx.clone();
        Html(self.0.render(name, &ctx).unwrap_or_else(|f| {
            let string = format!("Error description: {}, kind: {:?}", f, f.source());
            ctx.insert("reason", &string);
            self.0.render("PAGE_template_error.html", &ctx).unwrap()
        }))
    }
}

#[cfg(not(debug_assertions))]
fn tera_read() -> RwLockReadGuard<'static, Renderer> {
    TERA.read()
}

#[cfg(debug_assertions)]
fn tera_write() -> RwLockWriteGuard<'static, Renderer> {
    TERA.write()
}

pub fn render(name: &str, ctx: &Context) -> Page {
    #[cfg(debug_assertions)]
    {
        tera_write().html(name, ctx)
    }
    #[cfg(not(debug_assertions))]
    {
        tera_read().html(name, ctx)
    }
}

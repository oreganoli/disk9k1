use std::error::Error;

use crate::prelude::*;

/// A newtype that wraps `Tera` so we can avoid writing the `Html` wrapper and `.unwrap()`ping the value over and over.
pub struct Renderer(pub Tera);

impl Renderer {
    pub fn html(&self, name: &str, ctx: &Context) -> Page {
        Html(self.0.render(name, ctx).unwrap_or_else(|f| {
            let string = format!("Error description: {}, kind: {:?}", f, f.source());
            let mut ctx = Context::new();
            ctx.insert("reason", &string);
            self.0.render("template_error.html", &ctx).unwrap()
        }))
    }
}

pub fn mebibytes(bytes: u64) -> f64 {
    bytes as f64 / BYTES_TO_MEBIBYTE
}

/// Creates a `Pool` pointing at our database.
pub fn create_pool() -> Pool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set, panicking.");
    let manager = Manager::new(url);
    Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("Could not create a connection manager.")
}

/// A newtype wrapper around `Pool`s for easy `.get()`ting.
pub struct HandledPool(pub Pool);

impl HandledPool {
    pub fn new() -> Self {
        Self(create_pool())
    }
    pub fn get(&self) -> Connection {
        self.0
            .get()
            .expect("Could not obtain a connection, panicking.")
    }
}

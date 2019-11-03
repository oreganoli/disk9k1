use crate::prelude::*;

/// A newtype that wraps `Tera` so we can avoid writing the `Html` wrapper and `.unwrap()`ping the value over and over.
pub struct Renderer(pub Tera);

impl Renderer {
    pub fn html(&self, name: &str, ctx: &Context) -> Page {
        Html(self.0.render(name, ctx).unwrap())
    }
}
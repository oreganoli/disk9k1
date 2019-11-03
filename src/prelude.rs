pub use rocket::{response::content::Html, State};
pub use serde::{Deserialize, Serialize};
pub use tera::{Context, Tera};

pub use crate::file::File;
pub use crate::util::Renderer;

pub type LockState<'a> = State<'a, std::sync::RwLock<crate::instance::Instance>>;
pub type TeraState<'a> = State<'a, Renderer>;

pub type Page = Html<String>;
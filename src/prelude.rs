pub use rocket::{response::content::Html, State};
pub use serde::{Deserialize, Serialize};
pub use tera::{Context, Tera};

pub type LockState<'a, 'b> = State<'a, std::sync::RwLock<crate::instance::Instance<'b>>>;
pub type TeraState<'a> = State<'a, Tera>;
pub use chrono::NaiveDateTime;
pub use diesel::prelude::*;
pub use rocket::{
    http::{Cookie, Cookies},
    response::content::Html,
    State,
};
pub use serde::{Deserialize, Serialize};
pub use tera::{Context, Tera};

pub use crate::file::File;
pub use crate::schema;
pub use crate::util::{mebibytes, HandledPool, Renderer};

pub type LockState<'a> = State<'a, std::sync::RwLock<crate::instance::Instance>>;
pub type TeraState<'a> = State<'a, Renderer>;

pub type Page = Html<String>;

pub const BCRYPT_COST: u32 = 4;
pub const BYTES_TO_MEBIBYTE: f64 = 1048576.;

/// Shorthand alias for a pooled `r2d2` connection.
pub type Connection = diesel::r2d2::PooledConnection<Manager>;
/// Shorthand alias for a Postgres connection manager provided by `diesel::r2d2`.
pub type Manager = diesel::r2d2::ConnectionManager<PgConnection>;
/// Shorthand alias for a `diesel::r2d2::Pool`.
pub type Pool = diesel::r2d2::Pool<Manager>;

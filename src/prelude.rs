pub use std::default::Default;
pub use std::fmt::Display;
pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use chrono::NaiveDateTime;
pub use diesel::prelude::*;
pub use rocket::{
    http::{Cookie, Cookies, Status},
    request::{Form, FromForm},
    response::content::Html,
    response::status,
    response::Redirect,
    State,
};
pub use rocket_contrib::json::Json;
pub use serde::{Deserialize, Serialize};

pub use crate::error::{
    AppError, AuthError, DeletionError, DirectoryError, EmailChangeError, InstanceError,
    PasswordChangeError, RegistrationError, UserError,
};
pub use crate::instance::{Instance, InstanceData};
pub use crate::schema;
pub use crate::user::{NewUser, User, UserInfo};
pub use crate::util::lock::Lock;
pub use crate::util::pool::HandledPool;

pub type AppState<'a> = State<'a, Lock<Instance>>;
pub type AppResult<T> = Result<T, AppError>;

pub const BCRYPT_COST: u32 = 4;
pub const BYTES_TO_MEBIBYTE: f64 = 1_048_576f64;

/// Shorthand alias for a pooled `r2d2` connection.
pub type Connection = diesel::r2d2::PooledConnection<Manager>;
/// Shorthand alias for a Postgres connection manager provided by `diesel::r2d2`.
pub type Manager = diesel::r2d2::ConnectionManager<PgConnection>;
/// Shorthand alias for a `diesel::r2d2::Pool`.
pub type Pool = diesel::r2d2::Pool<Manager>;

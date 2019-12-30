pub use std::default::Default;
pub use std::fmt::Display;
pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use chrono::NaiveDateTime;
pub use failure::{Error, Fail};
use r2d2_postgres::r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
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

pub use crate::app::App;
pub use crate::error::ErrorWrapper;
pub use crate::instance::{repo::InstanceRepo, Instance};
pub use crate::user::{repo::UserRepo, AuthError, User, UserError};
pub use crate::util::lock::Lock;

pub type AppState<'a> = State<'a, Lock<App>>;
pub type AppResult<T> = Result<T, ErrorWrapper>;

pub const BCRYPT_COST: u32 = 4;
pub const BYTES_TO_MEBIBYTE: f64 = 1_048_576f64;

pub type Conn = PooledConnection<PostgresConnectionManager<postgres::NoTls>>;
/// Shorthand alias for a `diesel::r2d2::Pool`.
pub type Pool = r2d2::Pool<PostgresConnectionManager<postgres::NoTls>>;

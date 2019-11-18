use crate::prelude::*;

pub mod lock;
pub mod pool;

pub fn mebibytes(bytes: u64) -> f64 {
    bytes as f64 / BYTES_TO_MEBIBYTE
}

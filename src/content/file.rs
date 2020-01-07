use crate::prelude::*;

pub mod multi;

pub struct NewFile {
    filename: String,
    public: bool,
    directory: Option<i32>,
    data: Vec<u8>,
}

pub enum FileError {
    NamingConflict,
    NoFileName,
    TooBig,
    ImproperForm,
}

use core::fmt::Error;

use serde::export::Formatter;

use crate::prelude::*;

#[derive(Debug)]
pub enum DirectoryError {
    CyclicReference,
    Nonexistent,
    NotSameOwner,
}

impl Display for DirectoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            Self::CyclicReference => "A directory cannot be placed within itself.",
            Self::Nonexistent => "No such directory exists.",
            Self::NotSameOwner => "Attempted to create a new directory in, or move one into, a directory owned by another user."
        })
    }
}

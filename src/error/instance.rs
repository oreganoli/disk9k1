use serde::export::Formatter;

use crate::prelude::*;

/// The errors returned when the user is attempting to modify the instance's global settings.
#[derive(Debug)]
pub enum InstanceError {
    NameEmpty,
    NegativeSizeLimit,
}

impl Display for InstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::NameEmpty => "You must provide a name for the instance.",
                Self::NegativeSizeLimit =>
                    "The file size limit must be equal to or greater than 0 bytes.",
            }
        )
    }
}

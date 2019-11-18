use crate::prelude::*;

#[derive(Debug, Serialize)]
pub enum DirectoryError {
    CyclicReference,
    Nonexistent,
    NotSameOwner,
}

impl Error {
    pub fn dir<T>(de: DirectoryError) -> Result<T, Self> {
        Err(Self::Dir(de))
    }
}

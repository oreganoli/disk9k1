use super::Error;

#[derive(Debug)]
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

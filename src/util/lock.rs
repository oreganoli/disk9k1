use crate::prelude::*;

/// A newtype wrapper around `RwLocks` for easy `.read()`ing and `.write`ing.
pub struct Lock<T>(pub RwLock<T>);

impl<T> Lock<T> {
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap()
    }
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }
}

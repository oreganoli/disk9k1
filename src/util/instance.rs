use crate::prelude::*;
use crate::INSTANCE;

pub fn instance_read() -> RwLockReadGuard<'static, Instance> {
    INSTANCE.read()
}

pub fn instance_write() -> RwLockWriteGuard<'static, Instance> {
    INSTANCE.write()
}

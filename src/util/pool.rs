use crate::prelude::*;

/// Creates a `Pool` pointing at our database.
pub fn create_pool() -> Pool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set, panicking.");
    let manager = Manager::new(url);
    Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("Could not create a connection manager.")
}

/// A newtype wrapper around `Pool`s for easy `.get()`ting.
pub struct HandledPool(pub Pool);

impl HandledPool {
    pub fn new() -> Self {
        Self(create_pool())
    }
    pub fn get(&self) -> Connection {
        self.0
            .get()
            .expect("Could not obtain a connection, panicking.")
    }
}

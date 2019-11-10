use crate::prelude::*;
use crate::schema::instance;

use super::InstanceData;

/// The trait defining the interface for an `InstanceData` repository.
/// This allows us to easily create mocks for testing.
pub trait InstanceRepository {
    fn get(&self) -> Result<Option<InstanceData>, ()>;
    fn set(&mut self, data: InstanceData) -> Result<(), ()>;
}

/// Diesel-based repo for `InstanceData`
pub struct InstanceRepo {
    /// The SQL connection pool.
    pool: HandledPool,
    /// The value last set. Re-gotten after updates.
    cache: Option<InstanceData>,
}

impl InstanceRepo {
    pub fn new() -> Self {
        let mut repo = Self {
            pool: HandledPool::new(),
            cache: None,
        };
        repo.update_cache();
        repo
    }
    pub fn cache(&self) -> Option<InstanceData> {
        self.cache.clone()
    }
    pub fn update_cache(&mut self) {
        let conn = self.pool.get();
        instance::table.first(&conn).map_or_else(
            |err| match err {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(()),
            },
            |inst: InstanceData| Ok(Some(inst)),
        )
    }
}

impl InstanceRepository for InstanceRepo {
    fn get(&self) -> Result<Option<InstanceData>, ()> {
        Ok(self.cache())
    }
    fn set(&mut self, data: InstanceData) -> Result<(), ()> {
        let conn = self.pool.get();
        if let None = self.get().unwrap() {
            diesel::insert_into(instance::table)
                .values(data)
                .execute(&conn)
                .map_or_else(
                    |_| Err(()),
                    |_| {
                        self.update_cache();
                        Ok(())
                    },
                )
        } else {
            diesel::update(instance::table)
                .set((
                    instance::name.eq(data.name),
                    instance::description.eq(data.description),
                    instance::size_limit.eq(data.size_limit),
                ))
                .execute(&conn)
                .map_or_else(
                    |_| Err(()),
                    |_| {
                        self.update_cache();
                        Ok(())
                    },
                )
        }
    }
}

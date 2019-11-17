use crate::prelude::*;
use crate::schema::instance;

use super::InstanceData;

/// The trait defining the interface for an `InstanceData` repository.
/// This allows us to easily create mocks for testing.
pub trait InstanceRepository {
    fn get(&self) -> Result<Option<InstanceData>, Error>;
    fn set(&mut self, data: InstanceData) -> Result<(), Error>;
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
            pool: HandledPool::default(),
            cache: None,
        };
        repo.update_cache().unwrap();
        repo
    }
    pub fn cache(&self) -> Option<InstanceData> {
        self.cache.clone()
    }
    pub fn update_cache(&mut self) -> Result<(), Error> {
        let conn = self.pool.get();
        instance::table.first(&conn).map_or_else(
            |err| match err {
                diesel::result::Error::NotFound => Ok(()),
                _ => Err(Error::Db),
            },
            |inst: InstanceData| {
                self.cache = Some(inst);
                Ok(())
            },
        )
    }
}

impl InstanceRepository for InstanceRepo {
    fn get(&self) -> Result<Option<InstanceData>, Error> {
        Ok(self.cache())
    }
    fn set(&mut self, data: InstanceData) -> Result<(), Error> {
        let conn = self.pool.get();
        if self.get().unwrap().is_none() {
            diesel::insert_into(instance::table)
                .values(data)
                .execute(&conn)
                .map_or_else(|_| Err(Error::Db), |_| self.update_cache())
        } else {
            diesel::update(instance::table)
                .set((
                    instance::name.eq(data.name),
                    instance::description.eq(data.description),
                    instance::size_limit.eq(data.size_limit),
                ))
                .execute(&conn)
                .map_or_else(|_| Err(Error::Db), |_| self.update_cache())
        }
    }
}

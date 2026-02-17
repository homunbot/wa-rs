pub mod commands;
pub mod error;
pub mod persistence_manager;
pub mod signal;
pub mod signal_adapter;
pub mod traits;

// Re-export from the sqlite-storage crate when the feature is enabled
#[cfg(feature = "sqlite-storage")]
pub use wa_rs_sqlite_storage::SqliteStore;

pub use crate::store::traits::*;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Clone)]
pub struct Device {
    pub core: wa_rs_core::store::Device,
    pub backend: Arc<dyn Backend>,
}

impl Deref for Device {
    type Target = wa_rs_core::store::Device;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl DerefMut for Device {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}

impl Device {
    pub fn new(backend: Arc<dyn Backend>) -> Self {
        let core = wa_rs_core::store::Device::new();
        Self { core, backend }
    }

    pub fn to_serializable(&self) -> wa_rs_core::store::Device {
        self.core.clone()
    }

    pub fn load_from_serializable(&mut self, loaded: wa_rs_core::store::Device) {
        self.core = loaded;
    }
}

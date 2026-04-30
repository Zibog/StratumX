use crate::types::{PropertyType, WorldPropertySubstrate};

use super::PersistenceCodec;

/// Runtime substrate operations.
pub struct SubstrateRuntime {
    /// The substrate.
    pub substrate: WorldPropertySubstrate,
    /// Persistence codec.
    pub codec: PersistenceCodec,
    /// Dirty tracking for partial resume.
    pub dirty_properties: Vec<PropertyType>,
}

impl SubstrateRuntime {
    /// Create a new runtime with default substrate.
    pub fn new() -> Self {
        Self {
            substrate: WorldPropertySubstrate::with_default_update_order(),
            codec: PersistenceCodec::new(),
            dirty_properties: Vec::new(),
        }
    }
}

impl Default for SubstrateRuntime {
    fn default() -> Self {
        Self::new()
    }
}

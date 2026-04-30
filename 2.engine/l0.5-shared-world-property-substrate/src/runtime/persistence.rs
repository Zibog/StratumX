use crate::types::{PropertyType, WorldPropertySubstrate};

use super::SubstrateRuntime;

/// Persistence codec for world fields.
#[derive(Debug, Clone)]
pub struct PersistenceCodec {
    /// Enable compression.
    pub compress: bool,
    /// Enable partial resume (save only changed fields).
    pub partial_resume: bool,
}

impl PersistenceCodec {
    /// Create a new codec with default settings.
    pub fn new() -> Self {
        Self {
            compress: true,
            partial_resume: true,
        }
    }

    /// Serialize substrate to bytes.
    pub fn serialize(&self, substrate: &WorldPropertySubstrate) -> Result<Vec<u8>, String> {
        serde_json::to_vec(substrate).map_err(|e| format!("Serialization failed: {}", e))
    }

    /// Deserialize substrate from bytes.
    pub fn deserialize(&self, data: &[u8]) -> Result<WorldPropertySubstrate, String> {
        serde_json::from_slice(data).map_err(|e| format!("Deserialization failed: {}", e))
    }

    /// Serialize only changed fields (partial resume).
    pub fn serialize_partial(
        &self,
        substrate: &WorldPropertySubstrate,
        changed_properties: &[PropertyType],
    ) -> Result<Vec<u8>, String> {
        if !self.partial_resume {
            return self.serialize(substrate);
        }

        let mut partial = WorldPropertySubstrate::new();
        partial.update_order = substrate.update_order.clone();
        partial.conflict_rules = substrate.conflict_rules.clone();

        for &prop in changed_properties {
            if let Some(field) = substrate.cell_fields.get(&prop) {
                partial.cell_fields.insert(prop, field.clone());
            }
        }

        self.serialize(&partial)
    }

    /// Merge partial substrate into existing substrate.
    pub fn merge_partial(
        &self,
        substrate: &mut WorldPropertySubstrate,
        partial_data: &[u8],
    ) -> Result<(), String> {
        let partial = self.deserialize(partial_data)?;

        for (prop, field) in partial.cell_fields {
            substrate.cell_fields.insert(prop, field);
        }

        for (entity_id, fields) in partial.object_local_fields {
            substrate.object_local_fields.insert(entity_id, fields);
        }

        for (surface_id, fields) in partial.surface_fields {
            substrate.surface_fields.insert(surface_id, fields);
        }

        for (volume_id, field) in partial.volume_fields {
            substrate.volume_fields.insert(volume_id, field);
        }

        Ok(())
    }
}

impl Default for PersistenceCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl SubstrateRuntime {
    /// Save substrate to bytes.
    pub fn save(&self) -> Result<Vec<u8>, String> {
        self.codec.serialize(&self.substrate)
    }

    /// Save only changed fields (partial resume).
    pub fn save_partial(&self) -> Result<Vec<u8>, String> {
        self.codec
            .serialize_partial(&self.substrate, &self.dirty_properties)
    }

    /// Load substrate from bytes.
    pub fn load(&mut self, data: &[u8]) -> Result<(), String> {
        self.substrate = self.codec.deserialize(data)?;
        self.dirty_properties.clear();
        Ok(())
    }

    /// Load and merge partial substrate.
    pub fn load_partial(&mut self, data: &[u8]) -> Result<(), String> {
        self.codec.merge_partial(&mut self.substrate, data)?;
        self.dirty_properties.clear();
        Ok(())
    }

    /// Clear dirty tracking.
    pub fn clear_dirty(&mut self) {
        self.dirty_properties.clear();
    }
}

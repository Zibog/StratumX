/// Runtime operations for world property substrate
///
/// Provides update, query, and persistence operations for world fields.
use crate::types::*;

/// Persistence codec for world fields
#[derive(Debug, Clone)]
pub struct PersistenceCodec {
    /// Enable compression
    pub compress: bool,
    /// Enable partial resume (save only changed fields)
    pub partial_resume: bool,
}

impl PersistenceCodec {
    /// Create a new codec with default settings
    pub fn new() -> Self {
        Self {
            compress: true,
            partial_resume: true,
        }
    }

    /// Serialize substrate to bytes
    pub fn serialize(&self, substrate: &WorldPropertySubstrate) -> Result<Vec<u8>, String> {
        serde_json::to_vec(substrate).map_err(|e| format!("Serialization failed: {}", e))
    }

    /// Deserialize substrate from bytes
    pub fn deserialize(&self, data: &[u8]) -> Result<WorldPropertySubstrate, String> {
        serde_json::from_slice(data).map_err(|e| format!("Deserialization failed: {}", e))
    }

    /// Serialize only changed fields (partial resume)
    pub fn serialize_partial(
        &self,
        substrate: &WorldPropertySubstrate,
        changed_properties: &[PropertyType],
    ) -> Result<Vec<u8>, String> {
        if !self.partial_resume {
            return self.serialize(substrate);
        }

        // Create a partial substrate with only changed fields
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

    /// Merge partial substrate into existing substrate
    pub fn merge_partial(
        &self,
        substrate: &mut WorldPropertySubstrate,
        partial_data: &[u8],
    ) -> Result<(), String> {
        let partial = self.deserialize(partial_data)?;

        // Merge cell fields
        for (prop, field) in partial.cell_fields {
            substrate.cell_fields.insert(prop, field);
        }

        // Merge object-local fields
        for (entity_id, fields) in partial.object_local_fields {
            substrate.object_local_fields.insert(entity_id, fields);
        }

        // Merge surface fields
        for (surface_id, fields) in partial.surface_fields {
            substrate.surface_fields.insert(surface_id, fields);
        }

        // Merge volume fields
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

/// Runtime substrate operations
pub struct SubstrateRuntime {
    /// The substrate
    pub substrate: WorldPropertySubstrate,
    /// Persistence codec
    pub codec: PersistenceCodec,
    /// Dirty tracking for partial resume
    pub dirty_properties: Vec<PropertyType>,
}

impl SubstrateRuntime {
    /// Create a new runtime with default substrate
    pub fn new() -> Self {
        Self {
            substrate: WorldPropertySubstrate::with_default_update_order(),
            codec: PersistenceCodec::new(),
            dirty_properties: Vec::new(),
        }
    }

    /// Update a cell field value
    pub fn update_cell_field(
        &mut self,
        property: PropertyType,
        x: usize,
        y: usize,
        z: usize,
        value: f32,
    ) -> Result<(), String> {
        let field = self
            .substrate
            .cell_fields
            .get_mut(&property)
            .ok_or_else(|| format!("Cell field not found for property {:?}", property))?;

        let (dx, dy, dz) = field.dimensions;
        if x >= dx || y >= dy || z >= dz {
            return Err(format!("Coordinates ({}, {}, {}) out of bounds", x, y, z));
        }

        let index = x + y * dx + z * dx * dy;
        field.data[index] = value;

        // Mark as dirty for partial resume
        if !self.dirty_properties.contains(&property) {
            self.dirty_properties.push(property);
        }

        Ok(())
    }

    /// Query a cell field value
    pub fn query_cell_field(
        &self,
        property: PropertyType,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<f32, String> {
        let field = self
            .substrate
            .cell_fields
            .get(&property)
            .ok_or_else(|| format!("Cell field not found for property {:?}", property))?;

        let (dx, dy, dz) = field.dimensions;
        if x >= dx || y >= dy || z >= dz {
            return Err(format!("Coordinates ({}, {}, {}) out of bounds", x, y, z));
        }

        let index = x + y * dx + z * dx * dy;
        Ok(field.data[index])
    }

    /// Update properties in topological order
    pub fn update_all(&mut self) -> Result<(), String> {
        let order = self.substrate.update_order.topological_sort()?;

        // Process updates in dependency order
        for property in order {
            // Placeholder for actual update logic
            // In a real implementation, this would apply physics/simulation rules
            self.apply_property_update(property)?;
        }

        Ok(())
    }

    /// Apply update for a single property (placeholder)
    fn apply_property_update(&mut self, _property: PropertyType) -> Result<(), String> {
        // Placeholder for property-specific update logic
        // Real implementation would apply physics rules, diffusion, etc.
        Ok(())
    }

    /// Resolve conflicts between overlapping fields
    pub fn resolve_conflicts(&mut self) -> Result<(), String> {
        for rule in &self.substrate.conflict_rules.clone() {
            self.apply_conflict_rule(rule)?;
        }
        Ok(())
    }

    /// Apply a single conflict resolution rule
    fn apply_conflict_rule(&mut self, rule: &ConflictRule) -> Result<(), String> {
        match rule.resolution {
            ConflictResolution::Priority => {
                // First property in list wins
                // Implementation would prioritize values
            }
            ConflictResolution::Blend => {
                // Blend values using weights
                // Implementation would interpolate values
            }
            ConflictResolution::Maximum => {
                // Take maximum value
                // Implementation would compare and select max
            }
            ConflictResolution::Minimum => {
                // Take minimum value
                // Implementation would compare and select min
            }
        }
        Ok(())
    }

    /// Save substrate to bytes
    pub fn save(&self) -> Result<Vec<u8>, String> {
        self.codec.serialize(&self.substrate)
    }

    /// Save only changed fields (partial resume)
    pub fn save_partial(&self) -> Result<Vec<u8>, String> {
        self.codec
            .serialize_partial(&self.substrate, &self.dirty_properties)
    }

    /// Load substrate from bytes
    pub fn load(&mut self, data: &[u8]) -> Result<(), String> {
        self.substrate = self.codec.deserialize(data)?;
        self.dirty_properties.clear();
        Ok(())
    }

    /// Load and merge partial substrate
    pub fn load_partial(&mut self, data: &[u8]) -> Result<(), String> {
        self.codec.merge_partial(&mut self.substrate, data)?;
        self.dirty_properties.clear();
        Ok(())
    }

    /// Clear dirty tracking
    pub fn clear_dirty(&mut self) {
        self.dirty_properties.clear();
    }

    /// Initialize a cell field with given dimensions
    pub fn init_cell_field(
        &mut self,
        property: PropertyType,
        dimensions: (usize, usize, usize),
        initial_value: f32,
    ) {
        let (dx, dy, dz) = dimensions;
        let size = dx * dy * dz;
        let data = vec![initial_value; size];

        let field = CellField {
            property,
            dimensions,
            data,
        };

        self.substrate.cell_fields.insert(property, field);
    }
}

impl Default for SubstrateRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_field_update_and_query() {
        let mut runtime = SubstrateRuntime::new();
        runtime.init_cell_field(PropertyType::Wetness, (10, 10, 10), 0.0);

        // Update a cell
        runtime
            .update_cell_field(PropertyType::Wetness, 5, 5, 5, 0.75)
            .unwrap();

        // Query the cell
        let value = runtime
            .query_cell_field(PropertyType::Wetness, 5, 5, 5)
            .unwrap();
        assert_eq!(value, 0.75);

        // Verify dirty tracking
        assert!(runtime.dirty_properties.contains(&PropertyType::Wetness));
    }

    #[test]
    fn test_persistence_round_trip() {
        let mut runtime = SubstrateRuntime::new();
        runtime.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
        runtime
            .update_cell_field(PropertyType::Heat, 2, 2, 2, 350.0)
            .unwrap();

        // Save
        let data = runtime.save().unwrap();

        // Load into new runtime
        let mut runtime2 = SubstrateRuntime::new();
        runtime2.load(&data).unwrap();

        // Verify data
        let value = runtime2
            .query_cell_field(PropertyType::Heat, 2, 2, 2)
            .unwrap();
        assert_eq!(value, 350.0);
    }

    #[test]
    fn test_partial_resume() {
        let mut runtime = SubstrateRuntime::new();
        runtime.init_cell_field(PropertyType::Wetness, (5, 5, 5), 0.0);
        runtime.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);

        // Update only wetness
        runtime
            .update_cell_field(PropertyType::Wetness, 1, 1, 1, 0.5)
            .unwrap();

        // Save partial (only wetness should be saved)
        let partial_data = runtime.save_partial().unwrap();

        // Load partial into new runtime
        let mut runtime2 = SubstrateRuntime::new();
        runtime2.init_cell_field(PropertyType::Wetness, (5, 5, 5), 0.0);
        runtime2.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
        runtime2.load_partial(&partial_data).unwrap();

        // Verify wetness was updated
        let wetness = runtime2
            .query_cell_field(PropertyType::Wetness, 1, 1, 1)
            .unwrap();
        assert_eq!(wetness, 0.5);
    }

    #[test]
    fn test_update_order_execution() {
        let mut runtime = SubstrateRuntime::new();
        runtime.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
        runtime.init_cell_field(PropertyType::Wetness, (5, 5, 5), 0.5);

        // Update all should execute in topological order
        let result = runtime.update_all();
        assert!(result.is_ok());
    }
}

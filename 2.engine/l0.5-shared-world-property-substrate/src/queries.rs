/// Query interfaces for world property substrate
use crate::types::*;

/// Query interface for world property substrate
pub struct SubstrateQuery;

impl SubstrateQuery {
    /// Query all properties at a cell location
    pub fn query_cell_properties(
        substrate: &WorldPropertySubstrate,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(PropertyType, f32)> {
        let mut results = Vec::new();

        for (property, field) in &substrate.cell_fields {
            let (dx, dy, dz) = field.dimensions;
            if x < dx && y < dy && z < dz {
                let index = x + y * dx + z * dx * dy;
                if index < field.data.len() {
                    results.push((*property, field.data[index]));
                }
            }
        }

        results
    }

    /// Query a specific property at a cell location
    pub fn query_cell_property(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<f32> {
        let field = substrate.cell_fields.get(&property)?;
        let (dx, dy, dz) = field.dimensions;

        if x >= dx || y >= dy || z >= dz {
            return None;
        }

        let index = x + y * dx + z * dx * dy;
        field.data.get(index).copied()
    }

    /// Query all object-local fields for an entity
    pub fn query_object_fields(
        substrate: &WorldPropertySubstrate,
        entity_id: u64,
    ) -> Vec<ObjectLocalField> {
        substrate
            .object_local_fields
            .get(&entity_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Query a specific property for an entity
    pub fn query_object_property(
        substrate: &WorldPropertySubstrate,
        entity_id: u64,
        property: PropertyType,
    ) -> Option<f32> {
        let fields = substrate.object_local_fields.get(&entity_id)?;
        fields
            .iter()
            .find(|f| f.property == property)
            .map(|f| f.value)
    }

    /// Query all surface fields for a surface
    pub fn query_surface_fields(
        substrate: &WorldPropertySubstrate,
        surface_id: u64,
    ) -> Vec<SurfaceField> {
        substrate
            .surface_fields
            .get(&surface_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Query volume field
    pub fn query_volume_field(
        substrate: &WorldPropertySubstrate,
        volume_id: u64,
    ) -> Option<VolumeField> {
        substrate.volume_fields.get(&volume_id).cloned()
    }

    /// Query all properties of a specific type across all storage types
    pub fn query_property_everywhere(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> PropertyQueryResult {
        let mut result = PropertyQueryResult {
            property,
            cell_field: None,
            object_fields: Vec::new(),
            surface_fields: Vec::new(),
            volume_fields: Vec::new(),
        };

        // Cell field
        if let Some(field) = substrate.cell_fields.get(&property) {
            result.cell_field = Some(field.clone());
        }

        // Object-local fields
        for fields in substrate.object_local_fields.values() {
            for field in fields {
                if field.property == property {
                    result.object_fields.push(field.clone());
                }
            }
        }

        // Surface fields
        for fields in substrate.surface_fields.values() {
            for field in fields {
                if field.property == property {
                    result.surface_fields.push(field.clone());
                }
            }
        }

        // Volume fields
        for field in substrate.volume_fields.values() {
            if field.property == property {
                result.volume_fields.push(field.clone());
            }
        }

        result
    }

    /// Get update order for properties
    pub fn query_update_order(
        substrate: &WorldPropertySubstrate,
    ) -> Result<Vec<PropertyType>, String> {
        substrate.update_order.topological_sort()
    }

    /// Get all properties that depend on a given property
    pub fn query_dependents(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> Vec<PropertyType> {
        substrate
            .update_order
            .edges
            .iter()
            .filter(|(from, _)| *from == property)
            .map(|(_, to)| *to)
            .collect()
    }

    /// Get all properties that a given property depends on
    pub fn query_dependencies(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> Vec<PropertyType> {
        substrate
            .update_order
            .edges
            .iter()
            .filter(|(_, to)| *to == property)
            .map(|(from, _)| *from)
            .collect()
    }

    /// Get conflict rules involving a property
    pub fn query_conflict_rules(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> Vec<ConflictRule> {
        substrate
            .conflict_rules
            .iter()
            .filter(|rule| rule.properties.contains(&property))
            .cloned()
            .collect()
    }
}

/// Result of querying a property across all storage types
#[derive(Debug, Clone)]
pub struct PropertyQueryResult {
    pub property: PropertyType,
    pub cell_field: Option<CellField>,
    pub object_fields: Vec<ObjectLocalField>,
    pub surface_fields: Vec<SurfaceField>,
    pub volume_fields: Vec<VolumeField>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_substrate() -> WorldPropertySubstrate {
        let mut substrate = WorldPropertySubstrate::new();

        // Add cell field
        let cell_field = CellField {
            property: PropertyType::Wetness,
            dimensions: (5, 5, 5),
            data: vec![0.5; 125],
        };
        substrate
            .cell_fields
            .insert(PropertyType::Wetness, cell_field);

        // Add object-local field
        let obj_field = ObjectLocalField {
            property: PropertyType::Heat,
            entity_id: 1,
            value: 300.0,
        };
        substrate.object_local_fields.insert(1, vec![obj_field]);

        // Add update order
        substrate
            .update_order
            .add_edge(PropertyType::Heat, PropertyType::Wetness);

        substrate
    }

    #[test]
    fn test_query_cell_property() {
        let substrate = create_test_substrate();
        let value = SubstrateQuery::query_cell_property(&substrate, PropertyType::Wetness, 2, 2, 2);
        assert_eq!(value, Some(0.5));

        let value = SubstrateQuery::query_cell_property(&substrate, PropertyType::Heat, 2, 2, 2);
        assert_eq!(value, None); // Heat is not in cell fields
    }

    #[test]
    fn test_query_cell_properties() {
        let substrate = create_test_substrate();
        let properties = SubstrateQuery::query_cell_properties(&substrate, 2, 2, 2);
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].0, PropertyType::Wetness);
        assert_eq!(properties[0].1, 0.5);
    }

    #[test]
    fn test_query_object_property() {
        let substrate = create_test_substrate();
        let value = SubstrateQuery::query_object_property(&substrate, 1, PropertyType::Heat);
        assert_eq!(value, Some(300.0));

        let value = SubstrateQuery::query_object_property(&substrate, 2, PropertyType::Heat);
        assert_eq!(value, None); // Entity 2 doesn't exist
    }

    #[test]
    fn test_query_property_everywhere() {
        let substrate = create_test_substrate();
        let result = SubstrateQuery::query_property_everywhere(&substrate, PropertyType::Wetness);

        assert!(result.cell_field.is_some());
        assert_eq!(result.object_fields.len(), 0);
        assert_eq!(result.surface_fields.len(), 0);
        assert_eq!(result.volume_fields.len(), 0);
    }

    #[test]
    fn test_query_update_order() {
        let substrate = create_test_substrate();
        let order = SubstrateQuery::query_update_order(&substrate).unwrap();

        // Heat should come before Wetness
        let heat_idx = order.iter().position(|&p| p == PropertyType::Heat).unwrap();
        let wetness_idx = order
            .iter()
            .position(|&p| p == PropertyType::Wetness)
            .unwrap();
        assert!(heat_idx < wetness_idx);
    }

    #[test]
    fn test_query_dependents() {
        let substrate = create_test_substrate();
        let dependents = SubstrateQuery::query_dependents(&substrate, PropertyType::Heat);
        assert_eq!(dependents, vec![PropertyType::Wetness]);
    }

    #[test]
    fn test_query_dependencies() {
        let substrate = create_test_substrate();
        let dependencies = SubstrateQuery::query_dependencies(&substrate, PropertyType::Wetness);
        assert_eq!(dependencies, vec![PropertyType::Heat]);
    }
}

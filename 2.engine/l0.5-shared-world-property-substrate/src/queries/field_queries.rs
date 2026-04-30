use crate::types::{CellField, ObjectLocalField, PropertyType, WorldPropertySubstrate};

use super::SubstrateQuery;

impl SubstrateQuery {
    /// Query all properties at a cell location.
    pub fn query_cell_properties(
        substrate: &WorldPropertySubstrate,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(PropertyType, f32)> {
        let mut results = Vec::new();

        for (property, field) in &substrate.cell_fields {
            if let Some(index) = cell_index(field, x, y, z) {
                results.push((*property, field.data[index]));
            }
        }

        results
    }

    /// Query a specific property at a cell location.
    pub fn query_cell_property(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<f32> {
        let field = substrate.cell_fields.get(&property)?;
        cell_index(field, x, y, z).and_then(|index| field.data.get(index).copied())
    }

    /// Query all object-local fields for an entity.
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

    /// Query a specific property for an entity.
    pub fn query_object_property(
        substrate: &WorldPropertySubstrate,
        entity_id: u64,
        property: PropertyType,
    ) -> Option<f32> {
        let fields = substrate.object_local_fields.get(&entity_id)?;
        fields
            .iter()
            .find(|field| field.property == property)
            .map(|field| field.value)
    }
}

fn cell_index(field: &CellField, x: usize, y: usize, z: usize) -> Option<usize> {
    let (dx, dy, dz) = field.dimensions;
    if x >= dx || y >= dy || z >= dz {
        return None;
    }

    let index = x + y * dx + z * dx * dy;
    (index < field.data.len()).then_some(index)
}

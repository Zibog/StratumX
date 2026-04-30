use crate::types::{
    CellField, ObjectLocalField, PropertyType, SurfaceField, VolumeField, WorldPropertySubstrate,
};

use super::SubstrateQuery;

/// Result of querying a property across all storage types.
#[derive(Debug, Clone)]
pub struct PropertyQueryResult {
    pub property: PropertyType,
    pub cell_field: Option<CellField>,
    pub object_fields: Vec<ObjectLocalField>,
    pub surface_fields: Vec<SurfaceField>,
    pub volume_fields: Vec<VolumeField>,
}

impl SubstrateQuery {
    /// Query all properties of a specific type across all storage types.
    pub fn query_property_everywhere(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> PropertyQueryResult {
        let mut result = PropertyQueryResult {
            property,
            cell_field: substrate.cell_fields.get(&property).cloned(),
            object_fields: Vec::new(),
            surface_fields: Vec::new(),
            volume_fields: Vec::new(),
        };

        for fields in substrate.object_local_fields.values() {
            for field in fields {
                if field.property == property {
                    result.object_fields.push(field.clone());
                }
            }
        }

        for fields in substrate.surface_fields.values() {
            for field in fields {
                if field.property == property {
                    result.surface_fields.push(field.clone());
                }
            }
        }

        for field in substrate.volume_fields.values() {
            if field.property == property {
                result.volume_fields.push(field.clone());
            }
        }

        result
    }
}

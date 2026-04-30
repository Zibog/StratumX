use crate::types::{SurfaceField, VolumeField, WorldPropertySubstrate};

use super::SubstrateQuery;

impl SubstrateQuery {
    /// Query all surface fields for a surface.
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

    /// Query volume field.
    pub fn query_volume_field(
        substrate: &WorldPropertySubstrate,
        volume_id: u64,
    ) -> Option<VolumeField> {
        substrate.volume_fields.get(&volume_id).cloned()
    }
}

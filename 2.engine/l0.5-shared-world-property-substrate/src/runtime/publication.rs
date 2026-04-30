use crate::types::PropertyType;

use super::{cell_field_not_found, coordinates_out_of_bounds, SubstrateRuntime};

impl SubstrateRuntime {
    /// Query a cell field value.
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
            .ok_or_else(|| cell_field_not_found(property))?;

        let (dx, dy, dz) = field.dimensions;
        if x >= dx || y >= dy || z >= dz {
            return Err(coordinates_out_of_bounds(x, y, z));
        }

        let index = x + y * dx + z * dx * dy;
        Ok(field.data[index])
    }
}

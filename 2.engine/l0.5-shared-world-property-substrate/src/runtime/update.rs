use crate::{
    types::{CellField, PropertyType},
    validation::{validate_conflict_rule, validate_field_value},
};

use super::{cell_field_not_found, coordinates_out_of_bounds, SubstrateRuntime};

mod conflicts;
mod dependencies;

impl SubstrateRuntime {
    /// Update a cell field value.
    pub fn update_cell_field(
        &mut self,
        property: PropertyType,
        x: usize,
        y: usize,
        z: usize,
        value: f32,
    ) -> Result<(), String> {
        validate_field_value(property, value).map_err(|err| err.to_string())?;
        let field = self
            .substrate
            .cell_fields
            .get_mut(&property)
            .ok_or_else(|| cell_field_not_found(property))?;

        let (dx, dy, dz) = field.dimensions;
        if x >= dx || y >= dy || z >= dz {
            return Err(coordinates_out_of_bounds(x, y, z));
        }

        let index = x + y * dx + z * dx * dy;
        field.data[index] = value;
        self.mark_dirty(property);
        Ok(())
    }

    /// Update properties in topological order.
    pub fn update_all(&mut self) -> Result<(), String> {
        for property in self.substrate.update_order.topological_sort()? {
            self.apply_property_update(property)?;
        }
        Ok(())
    }

    /// Resolve conflicts between overlapping fields.
    pub fn resolve_conflicts(&mut self) -> Result<(), String> {
        for rule in &self.substrate.conflict_rules.clone() {
            validate_conflict_rule(rule).map_err(|err| err.to_string())?;
            self.apply_conflict_rule(rule)?;
        }
        Ok(())
    }

    /// Initialize a cell field with given dimensions.
    pub fn init_cell_field(
        &mut self,
        property: PropertyType,
        dimensions: (usize, usize, usize),
        initial_value: f32,
    ) {
        let size = dimensions.0 * dimensions.1 * dimensions.2;
        let field = CellField {
            property,
            dimensions,
            data: vec![initial_value; size],
        };

        self.substrate.cell_fields.insert(property, field);
    }

    fn mark_dirty(&mut self, property: PropertyType) {
        if !self.dirty_properties.contains(&property) {
            self.dirty_properties.push(property);
        }
    }
}

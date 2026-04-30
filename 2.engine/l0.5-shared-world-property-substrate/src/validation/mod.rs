//! Validation for world property substrate operations.

mod errors;
mod field_scope;
mod material_pair;
mod schema;

pub use errors::SubstrateValidationError;
pub use field_scope::{validate_coordinates, validate_dimensions, validate_field_value};
pub use material_pair::validate_conflict_rule;
pub use schema::validate_update_order;

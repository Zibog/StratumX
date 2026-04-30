//! Runtime operations for world property substrate.
//!
//! Provides update, query, publication, and persistence operations for world
//! fields.

use crate::types::PropertyType;

mod frame;
mod persistence;
mod publication;
mod update;

pub use frame::SubstrateRuntime;
pub use persistence::PersistenceCodec;

pub(super) fn cell_field_not_found(property: PropertyType) -> String {
    format!("Cell field not found for property {:?}", property)
}

pub(super) fn coordinates_out_of_bounds(x: usize, y: usize, z: usize) -> String {
    format!("Coordinates ({}, {}, {}) out of bounds", x, y, z)
}

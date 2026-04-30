//! World property types for unified field substrate.
//!
//! This module groups the public substrate types by role while preserving the
//! flat crate-level API through `pub use`.

mod events;
mod field;
mod handles;
mod material;
mod surface;

pub use events::WorldPropertySubstrate;
pub use field::{CellField, FieldStorage, FieldValue, PropertyType, UpdateOrderGraph};
pub use handles::ObjectLocalField;
pub use material::{ConflictResolution, ConflictRule};
pub use surface::{SurfaceField, VolumeField};

//! Spatial validation and error types.

use std::fmt;

/// Spatial operation validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpatialValidationError {
    /// Coordinates are in different spatial contexts and cannot be compared directly.
    IncompatibleCoordinateSpaces,
    /// Transform contains invalid scale (zero or negative).
    InvalidTransformScale,
    /// Chunk radius for halo operation is invalid.
    InvalidHaloRadius,
}

impl fmt::Display for SpatialValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleCoordinateSpaces => {
                write!(f, "Coordinates are in incompatible coordinate spaces")
            }
            Self::InvalidTransformScale => {
                write!(f, "Transform scale must be positive")
            }
            Self::InvalidHaloRadius => {
                write!(f, "Halo radius must be valid")
            }
        }
    }
}

impl std::error::Error for SpatialValidationError {}

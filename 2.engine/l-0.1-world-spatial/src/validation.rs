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
    /// Precision zone is incompatible with the requested spatial carrier.
    InvalidPrecisionZone,
    /// Rebase publication must describe a real anchor move.
    InvalidRebaseDelta,
    /// Far phenomenon track is missing required identity.
    InvalidFarPhenomenonTrack,
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
            Self::InvalidPrecisionZone => {
                write!(f, "Precision zone is incompatible with this carrier")
            }
            Self::InvalidRebaseDelta => {
                write!(
                    f,
                    "Rebase publication requires distinct source and target anchors"
                )
            }
            Self::InvalidFarPhenomenonTrack => {
                write!(
                    f,
                    "Far phenomenon track requires stable identity and family tag"
                )
            }
        }
    }
}

impl std::error::Error for SpatialValidationError {}

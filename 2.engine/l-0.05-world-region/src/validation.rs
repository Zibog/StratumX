//! Validation and error types for regions.

use crate::RegionAddress;
use std::fmt;

/// Region validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionValidationError {
    /// Region bounds are invalid (min >= max).
    InvalidBounds {
        min_x: i32,
        min_z: i32,
        max_x: i32,
        max_z: i32,
    },
    /// Region address conflicts with existing region.
    DuplicateRegionAddress { address: RegionAddress },
    /// Version epoch is invalid.
    InvalidEpoch { epoch: u64 },
    /// Region state transition is invalid.
    InvalidStateTransition {
        from: crate::RegionState,
        to: crate::RegionState,
    },
}

impl fmt::Display for RegionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds {
                min_x,
                min_z,
                max_x,
                max_z,
            } => {
                write!(
                    f,
                    "Invalid region bounds: min({}, {}) >= max({}, {})",
                    min_x, min_z, max_x, max_z
                )
            }
            Self::DuplicateRegionAddress { address } => {
                write!(f, "Duplicate region address: {:?}", address)
            }
            Self::InvalidEpoch { epoch } => {
                write!(f, "Invalid version epoch: {}", epoch)
            }
            Self::InvalidStateTransition { from, to } => {
                write!(f, "Invalid state transition: {:?} -> {:?}", from, to)
            }
        }
    }
}

impl std::error::Error for RegionValidationError {}

/// Validates region bounds are well-formed.
pub fn validate_bounds(
    min_x: i32,
    min_z: i32,
    max_x: i32,
    max_z: i32,
) -> Result<(), RegionValidationError> {
    if min_x >= max_x || min_z >= max_z {
        return Err(RegionValidationError::InvalidBounds {
            min_x,
            min_z,
            max_x,
            max_z,
        });
    }
    Ok(())
}

/// Validates region state transitions are legal.
pub fn validate_state_transition(
    from: crate::RegionState,
    to: crate::RegionState,
) -> Result<(), RegionValidationError> {
    use crate::RegionState::*;

    // Legal transitions:
    // Inactive -> Active, StreamingIn, Frozen
    // Active -> StreamingOut, Frozen
    // StreamingIn -> Active, Frozen
    // StreamingOut -> Inactive, Frozen
    // Frozen -> (any)
    let is_valid = matches!(
        (from, to),
        (Inactive, Active)
            | (Inactive, StreamingIn)
            | (Inactive, Frozen)
            | (Active, StreamingOut)
            | (Active, Frozen)
            | (StreamingIn, Active)
            | (StreamingIn, Frozen)
            | (StreamingOut, Inactive)
            | (StreamingOut, Frozen)
            | (Frozen, _)
    );

    if is_valid {
        Ok(())
    } else {
        Err(RegionValidationError::InvalidStateTransition { from, to })
    }
}

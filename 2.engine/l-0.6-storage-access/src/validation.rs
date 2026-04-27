use crate::types::{AccessDescriptor, AccessMode};
use engine_core::{EngineCoreError, EngineCoreResult};
use engine_storage_layout::LocalityClass;

/// Validate that READ mode is set on descriptor.
pub fn validate_read_mode(descriptor: &AccessDescriptor) -> EngineCoreResult<()> {
    if !descriptor.mode.contains(AccessMode::READ) {
        return Err(EngineCoreError::InvalidDescriptor(
            "read view requires READ mode",
        ));
    }
    Ok(())
}

/// Validate that WRITE or STAGED mode is set on descriptor.
pub fn validate_write_capable(descriptor: &AccessDescriptor) -> EngineCoreResult<()> {
    let can_write = descriptor
        .mode
        .intersects(AccessMode::WRITE | AccessMode::STAGED);
    if !can_write {
        return Err(EngineCoreError::InvalidDescriptor(
            "write window requires WRITE or STAGED mode",
        ));
    }
    Ok(())
}

/// Validate that direct write entries have proper staging setup.
pub fn validate_write_staging(descriptor: &AccessDescriptor) -> EngineCoreResult<()> {
    if descriptor.mode.contains(AccessMode::WRITE) && !descriptor.staged_mutation_handoff {
        return Err(EngineCoreError::InvalidDescriptor(
            "direct write entry is illegal; staged handoff required",
        ));
    }
    Ok(())
}

/// Validate traversal entry binding invariants.
pub fn validate_traversal_entry(
    descriptor: &AccessDescriptor,
    cache_hit: bool,
    attempted_recompile: bool,
    requested_locality: LocalityClass,
) -> EngineCoreResult<()> {
    if cache_hit && attempted_recompile {
        return Err(EngineCoreError::InvalidDescriptor(
            "cache hit must not trigger ad hoc compile",
        ));
    }
    if descriptor.locality != requested_locality {
        return Err(EngineCoreError::InvalidDescriptor(
            "locality class may not widen after bind",
        ));
    }
    Ok(())
}

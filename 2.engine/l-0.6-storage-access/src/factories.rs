use crate::runtime::{ReadView, WriteWindow};
use crate::types::AccessDescriptor;
use crate::validation::{
    validate_read_mode, validate_traversal_entry, validate_write_capable, validate_write_staging,
};
use engine_core::EngineCoreResult;
use engine_handle::StableEntityHandle;
use engine_storage_layout::LocalityClass;

/// Create a read-only view with validated access descriptor.
pub fn make_read_view(
    descriptor: AccessDescriptor,
    anchor: StableEntityHandle,
) -> EngineCoreResult<ReadView> {
    validate_read_mode(&descriptor)?;
    Ok(ReadView { descriptor, anchor })
}

/// Create a write-enabled window with validated access descriptor.
pub fn make_write_window(
    descriptor: AccessDescriptor,
    anchor: StableEntityHandle,
) -> EngineCoreResult<WriteWindow> {
    validate_write_capable(&descriptor)?;
    validate_write_staging(&descriptor)?;
    Ok(WriteWindow { descriptor, anchor })
}

/// Create an access view with full traversal entry validation.
#[allow(dead_code)]
pub fn create_traversal_view(
    descriptor: AccessDescriptor,
    anchor: StableEntityHandle,
    cache_hit: bool,
    attempted_recompile: bool,
    requested_locality: LocalityClass,
) -> EngineCoreResult<ReadView> {
    validate_traversal_entry(
        &descriptor,
        cache_hit,
        attempted_recompile,
        requested_locality,
    )?;
    validate_read_mode(&descriptor)?;
    Ok(ReadView { descriptor, anchor })
}

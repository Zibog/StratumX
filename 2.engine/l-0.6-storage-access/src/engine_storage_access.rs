use bitflags::bitflags;
use engine_core::{EngineCoreError, EngineCoreResult};
use engine_handle::StableEntityHandle;
use engine_storage_layout::LocalityClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraversalPlanId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScratchClass {
    Borrowed,
    Owned,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AccessMode: u8 {
        const READ = 0b0001;
        const WRITE = 0b0010;
        const STAGED = 0b0100;
        const MIXED = Self::READ.bits() | Self::STAGED.bits();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessDescriptor {
    pub mode: AccessMode,
    pub plan_id: TraversalPlanId,
    pub locality: LocalityClass,
    pub scratch: ScratchClass,
    pub staged_mutation_handoff: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadView {
    pub descriptor: AccessDescriptor,
    pub anchor: StableEntityHandle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteWindow {
    pub descriptor: AccessDescriptor,
    pub anchor: StableEntityHandle,
}

pub fn make_read_view(
    descriptor: AccessDescriptor,
    anchor: StableEntityHandle,
) -> EngineCoreResult<ReadView> {
    if !descriptor.mode.contains(AccessMode::READ) {
        return Err(EngineCoreError::InvalidDescriptor(
            "read view requires READ mode",
        ));
    }
    Ok(ReadView { descriptor, anchor })
}

pub fn make_write_window(
    descriptor: AccessDescriptor,
    anchor: StableEntityHandle,
) -> EngineCoreResult<WriteWindow> {
    let can_write = descriptor
        .mode
        .intersects(AccessMode::WRITE | AccessMode::STAGED);
    if !can_write {
        return Err(EngineCoreError::InvalidDescriptor(
            "write window requires WRITE or STAGED mode",
        ));
    }
    if descriptor.mode.contains(AccessMode::WRITE) && !descriptor.staged_mutation_handoff {
        return Err(EngineCoreError::InvalidDescriptor(
            "direct write entry is illegal; staged handoff required",
        ));
    }

    Ok(WriteWindow { descriptor, anchor })
}

pub fn traversal_entry_bind(
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

use crate::types::AccessDescriptor;
use engine_handle::StableEntityHandle;

/// Read-only view into storage accessed via access descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadView {
    /// Access authorization descriptor.
    pub descriptor: AccessDescriptor,
    /// Root entity anchor for traversal.
    pub anchor: StableEntityHandle,
}

/// Write-enabled window into storage accessed via access descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteWindow {
    /// Access authorization descriptor.
    pub descriptor: AccessDescriptor,
    /// Root entity anchor for traversal.
    pub anchor: StableEntityHandle,
}

impl ReadView {
    /// Get the access descriptor for this view.
    pub fn descriptor(&self) -> &AccessDescriptor {
        &self.descriptor
    }

    /// Get the anchor entity handle.
    pub fn anchor(&self) -> StableEntityHandle {
        self.anchor
    }
}

impl WriteWindow {
    /// Get the access descriptor for this window.
    pub fn descriptor(&self) -> &AccessDescriptor {
        &self.descriptor
    }

    /// Get the anchor entity handle.
    pub fn anchor(&self) -> StableEntityHandle {
        self.anchor
    }

    /// Check if staged mutation handoff is enabled.
    pub fn has_mutation_handoff(&self) -> bool {
        self.descriptor.staged_mutation_handoff
    }
}

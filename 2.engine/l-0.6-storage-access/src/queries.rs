use crate::types::{AccessDescriptor, AccessMode};

/// Check if descriptor has READ mode enabled.
pub fn has_read_mode(descriptor: &AccessDescriptor) -> bool {
    descriptor.mode.contains(AccessMode::READ)
}

/// Check if descriptor has WRITE mode enabled.
pub fn has_write_mode(descriptor: &AccessDescriptor) -> bool {
    descriptor.mode.contains(AccessMode::WRITE)
}

/// Check if descriptor has STAGED mode enabled.
pub fn has_staged_mode(descriptor: &AccessDescriptor) -> bool {
    descriptor.mode.contains(AccessMode::STAGED)
}

/// Check if descriptor allows any write operation.
pub fn is_write_capable(descriptor: &AccessDescriptor) -> bool {
    descriptor
        .mode
        .intersects(AccessMode::WRITE | AccessMode::STAGED)
}

/// Get permission summary for a descriptor.
pub fn access_summary(descriptor: &AccessDescriptor) -> String {
    let mut perms = Vec::new();
    if has_read_mode(descriptor) {
        perms.push("READ");
    }
    if has_write_mode(descriptor) {
        perms.push("WRITE");
    }
    if has_staged_mode(descriptor) {
        perms.push("STAGED");
    }
    format!("[{}]", perms.join("|"))
}

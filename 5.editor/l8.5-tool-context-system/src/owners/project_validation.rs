//! Project validation — legality checks and blockers.
//!
//! This module is the owner-local seam for project consistency checks.
//! The current implementation is intentionally minimal and can grow to cover:
//! - Verifying that project identity is complete
//! - Ensuring workspace identity is valid
//! - Checking save generation consistency
//! - Validating snapshot integrity

use super::project_owner::ProjectOwner;

impl ProjectOwner {
    /// Returns `true` if the project state is internally consistent.
    ///
    /// Today this reports the owner as structurally valid.
    pub fn is_valid(&self) -> bool {
        true
    }
}

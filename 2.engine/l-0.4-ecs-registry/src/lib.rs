//! Registry ownership and registration.
//!
//! ## Crate Invariants
//! - Registry entries are unique per key
//! - Membership descriptors are immutable once registered
//! - Registry operations are atomic
//! - No dangling references to unregistered entries

pub mod types;
pub use types::{MembershipDescriptor, RegistryModel};

pub mod runtime;

pub mod validation;
pub use validation::RegistryError;

pub mod queries;
pub use queries::RegistryQuery;

pub mod exports;

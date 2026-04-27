// ============================================================
// Engine Handle Refs — unified session/object/runtime/identity/state/artifact types
// ============================================================
//
//! Stable engine handles and refs for upper layers.
//!
//! This crate defines the canonical handle and reference types used by upper layers
//! to identify sessions, objects, runtimes, identities, state, and artifacts across
//! the SDK bridge boundary. All handle types are serialization-safe and carry
//! version discipline from `sdk_compat`.

mod artifact_refs;
mod identity_refs;
mod object_handles;
mod runtime_handles;
mod session_handles;
mod state_refs;

pub use artifact_refs::*;
pub use identity_refs::*;
pub use object_handles::*;
pub use runtime_handles::*;
pub use session_handles::*;
pub use state_refs::*;

/// Canonical SDK level for this crate.
pub const CANONICAL_LEVEL: &str = "l5.10";

/// Re-export compat version for handle versioning.
pub use sdk_compat::BridgeVersion;

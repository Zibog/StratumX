//! Audio Executor - audio authority operations
//!
//! All audio source, zone, ducking, and preview operations.
//! Split into sub-modules by role:
//! - source_binding: audio source position and acoustic profile binding
//! - profile_binding: zone profile and ducking policy binding
//! - preview: audio preview playback through runtime kernel
//! - integration_failures: snapshot, artifact, and world-route bookkeeping

mod integration_failures;
mod preview;
mod profile_binding;
mod source_binding;

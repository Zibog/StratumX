//! StratumX Editor - L9.7 Animation Cinematics Authoring Suite
//!
//! Provides animation and cinematics authoring capabilities including
//! timeline editing, keyframe animation, and cinematic sequence composition.

pub mod api;
pub mod timeline;
pub mod keyframe;
pub mod animation_clip;
pub mod cinematic_sequence;
mod model;
mod runtime;
mod validation;

// Re-export public API
pub use api::*;
pub use timeline::*;
pub use keyframe::*;
pub use animation_clip::*;
pub use cinematic_sequence::*;

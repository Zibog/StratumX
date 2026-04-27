//! Audio runtime - service logic and operations.
//!
//! Contains the AudioAuthoringService that owns audio truth and provides
//! the operational interface for managing audio sources, zones, and policies.

mod authoring_service;
mod binding_ops;
mod preview_ops;

pub use authoring_service::AudioAuthoringService;
pub use preview_ops::{
    preview_audibility, preview_obstruction_occlusion, preview_transition,
    preview_voice_subtitle_legality,
};

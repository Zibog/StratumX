#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]

// Editor DTO Law - Frozen Semantic DTOs
// Stack version: SX-CANON/1.0.20/STACK-v25
//
// This is the canonical DTO law for editor-engine communication.
// These types may NOT drift by host technology.
// Field set is frozen. Field meaning is frozen.

mod content_state;
mod diagnostics;
mod identity;
mod material_authority_view;
mod runtime_entry;
mod viewport_frames;
mod world_lifecycle;
mod world_package;

pub use content_state::*;
pub use diagnostics::*;
pub use identity::*;
pub use material_authority_view::*;
pub use runtime_entry::*;
pub use viewport_frames::*;
pub use world_lifecycle::*;
pub use world_package::*;

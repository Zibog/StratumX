//! Diagnostics domain types split by role.

pub mod events;
pub mod ids;
pub mod packets;
pub mod panels;
pub mod recovery;
pub mod severity;

pub use events::*;
pub use ids::*;
pub use packets::*;
pub use panels::*;
pub use recovery::*;
pub use severity::*;

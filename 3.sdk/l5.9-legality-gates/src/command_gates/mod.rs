//! Command-specific legality gates split by domain.

mod assets;
mod audio;
mod build;
mod common;
mod diagnostics;
mod editor;
mod errors;
mod graphics;
mod materials;
mod netcode;
mod verdict;

pub use assets::*;
pub use diagnostics::*;
pub use editor::*;
pub use graphics::*;
pub use materials::*;

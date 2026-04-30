//! FUTURE_STUB.
//!
//! This crate is present as a canonical future surface.
//! It is not part of the active product spine yet.
//! It must not be counted as product-complete.

mod model;
mod runtime;

// Public API
pub mod api;

// Re-export public API at crate root for backward compatibility
pub use api::*;

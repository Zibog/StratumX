//! Identity module — manages entity and component lifetime allocation, retirement, and epoch tracking.

pub mod exports;
pub mod queries;
pub mod runtime;
pub mod types;
pub mod validation;

pub use runtime::*;
pub use types::*;

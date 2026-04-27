//! Routing — command domain classification and routing decisions.
//!
//! Separated from execution per canon: routing ≠ execution.

pub mod command_domain;
pub mod command_router;

pub use command_domain::CommandDomain;
pub use command_router::CommandRouter;

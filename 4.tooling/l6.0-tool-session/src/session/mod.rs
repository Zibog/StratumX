//! Session mechanics — envelope storage, ID sources, lifecycle rules.
//!
//! Separated from domain execution per canon: session bookkeeping ≠ executor logic.

pub mod command_id_source;
pub mod command_lifecycle;
pub mod envelope_store;
pub mod request_id_source;

pub use command_id_source::CommandIdSource;
pub use command_lifecycle::CommandLifecycle;
pub use envelope_store::EnvelopeStore;
pub use request_id_source::RequestIdSource;

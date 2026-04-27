pub mod canonical_command_schema;
pub mod command_lifecycle;
pub mod promoted_commands;
pub mod validation;

pub use canonical_command_schema::*;
pub use command_lifecycle::{CommandEnvelope, CommandLifecycleState, CommandLifecycleTracker};
pub use promoted_commands::*;
pub use validation::*;

pub const CANONICAL_LEVEL: &str = "l6.1-command-envelopes";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L61CommandEnvelopesMarker;

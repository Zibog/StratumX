//! Command types — core type definitions.
//!
//! This module provides the core command types for the editor command spine,
//! including command IDs, action IDs, command types, parameters, results, and errors.

mod errors;
mod types;

pub use errors::{ExecutionError, PreconditionError, RedoError, UndoError, ValidationError};
pub use types::{
    ActionId, Command, CommandId, CommandParameters, CommandResult, CommandType, ExecutedCommand,
    UndoData,
};

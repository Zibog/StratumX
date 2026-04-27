//! Command Error Types

use serde::{Deserialize, Serialize};

use super::types::CommandId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    pub command_id: CommandId,
    pub reason: String,
}

impl ValidationError {
    pub fn new(cid: CommandId, r: impl Into<String>) -> Self {
        Self {
            command_id: cid,
            reason: r.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreconditionError {
    pub command_id: CommandId,
    pub failed_preconditions: Vec<String>,
}

impl PreconditionError {
    pub fn new(cid: CommandId, fp: Vec<String>) -> Self {
        Self {
            command_id: cid,
            failed_preconditions: fp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionError {
    pub command_id: CommandId,
    pub reason: String,
    pub recoverable: bool,
}

impl ExecutionError {
    pub fn new(cid: CommandId, r: impl Into<String>, rc: bool) -> Self {
        Self {
            command_id: cid,
            reason: r.into(),
            recoverable: rc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndoError {
    pub command_id: CommandId,
    pub reason: String,
}

impl UndoError {
    pub fn new(cid: CommandId, r: impl Into<String>) -> Self {
        Self {
            command_id: cid,
            reason: r.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedoError {
    pub command_id: CommandId,
    pub reason: String,
}

impl RedoError {
    pub fn new(cid: CommandId, r: impl Into<String>) -> Self {
        Self {
            command_id: cid,
            reason: r.into(),
        }
    }
}

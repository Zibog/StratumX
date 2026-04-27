// Tooling Session Types - Core types

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ObjectHandle(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ObjectClass {
    World,
    Scene,
    Terrain,
    Material,
    Logic,
    Asset,
    Build,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolingError {
    UnknownObject,
    NoBuildArtifact,
    Unsupported,
    Message(String),
    PreconditionFailed(DisabledReason),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisabledReason {
    NoWorldOpen,
    NoProjectOpen,
    MaterialAuthorityUnavailable,
    AudioAuthorityUnavailable,
    RuntimeKernelUnavailable,
    TerrainNotAvailable,
    InvalidInput(String),
}

impl From<String> for ToolingError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

impl core::fmt::Display for ToolingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownObject => write!(f, "unknown object"),
            Self::NoBuildArtifact => write!(f, "no build artifact"),
            Self::Unsupported => write!(f, "unsupported operation"),
            Self::Message(v) => write!(f, "{v}"),
            Self::PreconditionFailed(reason) => write!(f, "precondition failed: {:?}", reason),
        }
    }
}

impl std::error::Error for ToolingError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandOrigin {
    User,
    Automation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalClass {
    None,
    ReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetClass {
    Interactive,
    Background,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L60ToolSessionMarker;

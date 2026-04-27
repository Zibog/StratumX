use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DestructionError {
    UnknownField,
    UnknownOverlay,
    InvalidConfiguration(String),
    OverlayError(String),
    CaptureError(String),
    ComparisonError(String),
    Message(String),
}

impl From<String> for DestructionError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

impl std::fmt::Display for DestructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownField => write!(f, "Unknown destruction field"),
            Self::UnknownOverlay => write!(f, "Unknown overlay"),
            Self::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            Self::OverlayError(msg) => write!(f, "Overlay error: {}", msg),
            Self::CaptureError(msg) => write!(f, "Capture error: {}", msg),
            Self::ComparisonError(msg) => write!(f, "Comparison error: {}", msg),
            Self::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for DestructionError {}

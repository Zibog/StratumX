use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    EmptyComponentSet,
    NoAccessMode,
    WriteLacksScratch,
    ZeroCacheKey,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponentSet => write!(f, "Query component set must be non-empty"),
            Self::NoAccessMode => write!(f, "Query access mode must be explicit"),
            Self::WriteLacksScratch => write!(f, "Write queries require scratch class"),
            Self::ZeroCacheKey => write!(f, "Cache key must be non-zero"),
        }
    }
}

impl std::error::Error for QueryError {}

use thiserror::Error;

pub type EngineCoreResult<T> = Result<T, EngineCoreError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EngineCoreError {
    #[error("invalid descriptor: {0}")]
    InvalidDescriptor(&'static str),
    #[error("invariant violated: {0}")]
    InvariantViolation(&'static str),
    #[error("feature profile conflict: {0}")]
    FeatureConflict(&'static str),
}

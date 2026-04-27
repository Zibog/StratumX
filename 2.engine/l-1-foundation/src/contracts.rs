use crate::error::EngineCoreResult;

/// Minimal descriptor contract consumed by upper crates.
pub trait EngineDescriptor {
    fn validate(&self) -> EngineCoreResult<()>;
}

/// Tiny invariant helper surface for lower substrate crates.
pub trait Invariant {
    fn check_invariants(&self) -> EngineCoreResult<()>;
}

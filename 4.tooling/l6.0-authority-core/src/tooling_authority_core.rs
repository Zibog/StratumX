pub const CANONICAL_LEVEL: &str = "l6.0-authority-core";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L60AuthorityCoreMarker;

// Authority containers
pub mod containers;

// Diagnostics and recovery
pub mod diagnostics;
pub mod recovery;

// Tooling conveyor (import/cook/certification)
pub mod conveyor;

// Layer purity enforcement
pub mod layer_purity;

// Re-exports
pub use conveyor::{
    CertificationPipeline, ConveyorItem, ConveyorStage, CookPipeline, ImportPipeline,
    ToolingConveyor,
};
pub use diagnostics::{
    AuthorityDiagnostic, AuthorityType, DiagnosticPublisher, DiagnosticSeverity,
    StderrDiagnosticPublisher,
};
pub use recovery::{
    ErrorClass, ErrorState, RecoveryContext, RecoveryManager, RecoveryStrategy, RecoveryTarget,
};

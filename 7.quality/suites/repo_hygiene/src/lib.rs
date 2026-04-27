// Repository Hygiene Suite Library
//
// This library provides data models and utilities for repository sanitization,
// including hygiene checking, test migration, and code quality enforcement.

pub mod cache_validator;
pub mod code_cleaner;
pub mod codebase_scanner;
pub mod discovery_scanner;
pub mod forbidden_shortcuts;
pub mod hygiene_checker;
pub mod migration_logger;
pub mod models;
pub mod owner_inventory;
pub mod ownership_validator;
pub mod persistence_validator;
pub mod quality_restructurer;
pub mod test_migrator;
pub mod ui_state_validator;
pub mod waiver_registry;

// Re-export commonly used types for convenience
pub use cache_validator::{CacheRebuildabilityValidator, CacheViolation, CacheViolationType};
pub use code_cleaner::CodeCleaner;
pub use codebase_scanner::{CodebaseState, StateField};
pub use discovery_scanner::DiscoveryScanner;
pub use forbidden_shortcuts::{
    ForbiddenShortcut, ForbiddenShortcutScanner, ForbiddenShortcutViolation,
};
pub use hygiene_checker::HygieneChecker;
pub use migration_logger::MigrationLogger;
pub use models::{
    CleanupReport, CompilationStatus, DomainLogicType, DomainLogicViolation, HostBypassPattern,
    HygieneReport, HygieneRule, HygieneViolation, MigrationResult, RegistrationBlob,
    TestFileCandidate, TestType, WaiverError,
};
pub use owner_inventory::{OwnerInventory, OwnerInventoryEntry, StateClassification};
pub use ownership_validator::{OwnershipCompletenessValidator, OwnershipViolation, ViolationType};
pub use persistence_validator::{
    PersistenceSeparationValidator, PersistenceViolation, PersistenceViolationType,
};
pub use quality_restructurer::QualityRestructurer;
pub use test_migrator::TestMigrator;
pub use ui_state_validator::{
    UiStateClassificationValidator, UiStateViolation, UiStateViolationType,
};
pub use waiver_registry::{WaiverEntry, WaiverRegistry};

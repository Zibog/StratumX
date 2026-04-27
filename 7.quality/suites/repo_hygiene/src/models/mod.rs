// Core data models for repository hygiene checking and test migration.

mod cleanup;
mod hygiene;
mod migration;
mod waiver;

pub use cleanup::{
    CleanupReport, DomainLogicType, DomainLogicViolation, HostBypassPattern, RegistrationBlob,
};
pub use hygiene::{HygieneReport, HygieneRule, HygieneViolation};
pub use migration::{CompilationStatus, MigrationResult, TestFileCandidate, TestType};
pub use waiver::WaiverError;

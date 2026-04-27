use std::path::PathBuf;
use std::time::Duration;

/// Represents a single hygiene rule violation detected in the codebase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HygieneViolation {
    pub path: PathBuf,
    pub rule: HygieneRule,
    pub line_number: Option<usize>,
    pub context: String,
    pub waived: bool,
    pub remediation: String,
}

/// Enumeration of all hygiene rules enforced by the checker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HygieneRule {
    LineLimit { limit: usize, actual: usize },
    TestFileInSrc,
    TodoComment,
    AllowAttribute { attr: String },
    DocAssetInRuntime,
    TestAssetInRuntime,
    TruthBypass,
    DuplicateModule { other_location: PathBuf },
    AppLogic,
    StaticMut,
    ExecuteBridge,
    HostBypass,
    HardcodedPath,
    FakeTruth,
}

/// Aggregated report of all hygiene check results.
#[derive(Debug, Clone)]
pub struct HygieneReport {
    pub violations: Vec<HygieneViolation>,
    pub passed_checks: usize,
    pub failed_checks: usize,
    pub execution_time: Duration,
}

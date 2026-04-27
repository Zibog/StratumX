use std::path::PathBuf;

/// Represents a host bypass pattern detected in UI code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBypassPattern {
    pub file: PathBuf,
    pub line: usize,
    pub pattern: String,
    pub suggested_action: String,
}

/// Represents a large registration module that should be decomposed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationBlob {
    pub file: PathBuf,
    pub line_count: usize,
    pub mixed_concerns: Vec<String>,
    pub suggested_decomposition: Vec<String>,
}

/// Represents domain logic found in an inappropriate layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainLogicViolation {
    pub file: PathBuf,
    pub violation_type: DomainLogicType,
    pub line_range: (usize, usize),
    pub suggested_target_layer: String,
}

/// Types of domain logic that should not be in UI code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainLogicType {
    Parser,
    Validator,
    BusinessRule,
}

/// Aggregated report of all cleanup operations needed.
#[derive(Debug, Clone)]
pub struct CleanupReport {
    pub host_bypasses: Vec<HostBypassPattern>,
    pub registration_blobs: Vec<RegistrationBlob>,
    pub domain_logic_violations: Vec<DomainLogicViolation>,
    pub total_issues: usize,
}

impl CleanupReport {
    pub fn new(
        host_bypasses: Vec<HostBypassPattern>,
        registration_blobs: Vec<RegistrationBlob>,
        domain_logic_violations: Vec<DomainLogicViolation>,
    ) -> Self {
        let total_issues =
            host_bypasses.len() + registration_blobs.len() + domain_logic_violations.len();
        Self {
            host_bypasses,
            registration_blobs,
            domain_logic_violations,
            total_issues,
        }
    }
}

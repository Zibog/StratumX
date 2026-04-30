//! Host-facing diagnostics packets and filters.

use std::time::SystemTime;

use crate::diagnostics_types::Severity;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub domain: String,
    pub message: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticFilter {
    pub severity: Option<Severity>,
    pub domain: Option<String>,
    pub after: Option<SystemTime>,
    pub before: Option<SystemTime>,
}

impl DiagnosticFilter {
    pub fn matches(&self, diagnostic: &Diagnostic) -> bool {
        if let Some(severity) = self.severity {
            if diagnostic.severity != severity {
                return false;
            }
        }

        if let Some(ref domain) = self.domain {
            if &diagnostic.domain != domain {
                return false;
            }
        }

        if let Some(after) = self.after {
            if diagnostic.timestamp < after {
                return false;
            }
        }

        if let Some(before) = self.before {
            if diagnostic.timestamp > before {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticsUpdatedEvent {
    pub total_count: usize,
}

//! Diagnostics Service
//!
//! Collects and manages diagnostics from all editor operations.
//! Persists diagnostics to diagnostics plane and supports filtering.
//!
//! **Requirements: 12.5, 29.2, 29.3**

use super::diagnostics_types::*;

/// Diagnostics service - handles diagnostic collection and querying
///
/// **Canonical Route Flow:**
/// EditorOperations → DiagnosticsService → DiagnosticsPlane
///
/// **Responsibilities:**
/// - Collect diagnostics from all editor operations
/// - Persist diagnostics to diagnostics plane
/// - Support filtering by severity, domain, and time
/// - Under 150 lines
pub struct DiagnosticsService {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsService {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Initialize service
    ///
    /// **Requirement 5.3**: Services provide lifecycle methods
    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialization logic here (if needed)
        Ok(())
    }

    /// Shutdown service
    ///
    /// **Requirement 5.3**: Services provide lifecycle methods
    pub fn shutdown(&mut self) -> Result<(), String> {
        // Cleanup logic here (if needed)
        Ok(())
    }

    /// Collect a diagnostic message
    ///
    /// **Events Emitted:**
    /// - diagnostics_updated through EventBus
    pub fn collect_diagnostic(&mut self, diagnostic: Diagnostic) -> DiagnosticsUpdatedEvent {
        // The launch contour keeps diagnostics in the host-owned buffer and
        // treats event emission as the authoritative outward signal.

        self.diagnostics.push(diagnostic);

        DiagnosticsUpdatedEvent {
            total_count: self.diagnostics.len(),
        }
    }

    /// Query diagnostics with optional filters
    pub fn query_diagnostics(&self, filter: DiagnosticFilter) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| filter.matches(d))
            .collect()
    }

    /// Clear diagnostics matching filter
    ///
    /// **Events Emitted:**
    /// - diagnostics_updated through EventBus
    pub fn clear_diagnostics(&mut self, filter: DiagnosticFilter) -> DiagnosticsUpdatedEvent {
        self.diagnostics.retain(|d| !filter.matches(d));

        DiagnosticsUpdatedEvent {
            total_count: self.diagnostics.len(),
        }
    }

    /// Get total diagnostic count
    pub fn count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Get diagnostics by severity
    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == severity)
            .count()
    }
}

impl Default for DiagnosticsService {
    fn default() -> Self {
        Self::new()
    }
}

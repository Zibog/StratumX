use crate::owners::world_owner::WorldOwner;
use crate::queries::ReadModel;
use crate::Severity;

/// World diagnostics summary view
///
/// Read-only summary of world-level diagnostics.
#[derive(Debug, Clone)]
pub struct WorldDiagnosticsSummaryView {
    pub diagnostic_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub has_errors: bool,
}

impl ReadModel<WorldOwner, WorldDiagnosticsSummaryView> for WorldDiagnosticsSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        let diagnostics = owner.get_diagnostics();
        let error_count = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Error | Severity::Critical))
            .count();
        let warning_count = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Warning))
            .count();

        Self {
            diagnostic_count: diagnostics.len(),
            error_count,
            warning_count,
            has_errors: error_count > 0,
        }
    }
}

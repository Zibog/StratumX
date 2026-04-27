use crate::owners::world_owner::WorldOwner;
use crate::queries::ReadModel;

/// Material coverage summary view
///
/// Read-only summary of material binding completeness.
/// This view would typically be built from MaterialAuthoringService state,
/// but since that's a domain service (not an owner), this demonstrates
/// the pattern for querying material coverage data.
#[derive(Debug, Clone)]
pub struct MaterialCoverageSummaryView {
    pub total_materials: usize,
    pub fully_bound_materials: usize,
    pub partially_bound_materials: usize,
    pub unbound_materials: usize,
    pub coverage_percentage: f32,
    pub materials_with_issues: Vec<MaterialIssue>,
}

/// Material issue information
#[derive(Debug, Clone)]
pub struct MaterialIssue {
    pub material_name: String,
    pub missing_bindings: Vec<String>,
    pub invalid_combinations: Vec<String>,
}

impl ReadModel<WorldOwner, MaterialCoverageSummaryView> for MaterialCoverageSummaryView {
    fn build(_owner: &WorldOwner) -> Self {
        // WorldOwner does not currently expose material coverage truth.
        // Represent that gap as an explicit empty summary rather than fabricating coverage.
        Self {
            total_materials: 0,
            fully_bound_materials: 0,
            partially_bound_materials: 0,
            unbound_materials: 0,
            coverage_percentage: 0.0,
            materials_with_issues: Vec::new(),
        }
    }
}

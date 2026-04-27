//! World query views
//!
//! Read-only views of WorldOwner state implementing ReadModel trait.

mod diagnostics_summary_view;
mod environment_summary_view;
mod identity_view;
mod material_coverage_view;
mod terrain_summary_view;

pub use diagnostics_summary_view::WorldDiagnosticsSummaryView;
pub use environment_summary_view::WorldEnvironmentSummaryView;
pub use identity_view::WorldIdentityView;
pub use material_coverage_view::{MaterialCoverageSummaryView, MaterialIssue};
pub use terrain_summary_view::WorldTerrainSummaryView;

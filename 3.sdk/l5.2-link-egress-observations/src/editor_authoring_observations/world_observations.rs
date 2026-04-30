use super::diagnostics::{
    DiagnosticEntryDto, PreviewStateDto, ViewportStatsDto, WorldSummaryDto, WorldTreeNodeDto,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorldObservation {
    WorldTree { nodes: Vec<WorldTreeNodeDto> },
    WorldSummary { summary: WorldSummaryDto },
    ViewportStats { stats: ViewportStatsDto },
    PreviewState { state: PreviewStateDto },
    Diagnostics { entries: Vec<DiagnosticEntryDto> },
    Error { message: String },
}

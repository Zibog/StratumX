use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldTreeNodeDto {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<WorldTreeNodeDto>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldSummaryDto {
    pub active_scene: Option<String>,
    pub entity_count: u32,
    pub active_actor: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewportStatsDto {
    pub fps: u32,
    pub frame_time_ms: f32,
    pub entity_count: u32,
    pub active_scene: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewStateDto {
    pub is_playing: bool,
    pub is_paused: bool,
    pub session_active: bool,
    pub active_scene: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticEntryDto {
    pub level: String,
    pub message: String,
    pub source: String,
    pub location: Option<String>,
}

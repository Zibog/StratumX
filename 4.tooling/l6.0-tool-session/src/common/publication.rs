use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationChange {
    pub button_id: String,
    pub owner_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorPublication {
    ShellChanged(PublicationChange),
    ProjectChanged(PublicationChange),
    WorldChanged(PublicationChange),
    TerrainChanged(PublicationChange),
    MaterialChanged(PublicationChange),
    EnvironmentChanged(PublicationChange),
    AudioChanged(PublicationChange),
    RuntimeChanged(PublicationChange),
    ValidationChanged(PublicationChange),
    DiagnosticsChanged(PublicationChange),
    EvidenceChanged(PublicationChange),
    BuildChanged(PublicationChange),
}

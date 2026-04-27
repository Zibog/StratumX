use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceSurface {
    MainMenu,
    ProjectWizard,
    OpenWorldDialog,
    TerrainPanel,
    MaterialPanel,
    SkyPanel,
    AudioPanel,
    BuildPanel,
    ViewportPanel,
    InspectorPanel,
    CommandPalette,
    AutomationPanel,
    QualityPanel,
    Unknown,
}

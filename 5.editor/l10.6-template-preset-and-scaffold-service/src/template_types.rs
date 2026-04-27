use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplatePresetAndScaffoldService {
    pub template_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorTemplateKind {
    OpenWorldScaffold,
    TerrainMaterialStack,
    QuestGameplaySlice,
    WeatherEnvironmentProfile,
    AudioUiPack,
}

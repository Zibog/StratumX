#[derive(Debug, Clone, Default)]
pub struct ToolSessionContext {
    pub executed_buttons: Vec<String>,
    pub focus_history: Vec<String>,
    pub diagnostics_notes: Vec<String>,
}

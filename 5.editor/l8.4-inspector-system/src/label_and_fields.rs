use crate::*;

impl EditorProduct {
    pub fn set_selected_label(&mut self, label: impl Into<String>) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::SetLabel {
                handle,
                label: label.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
    pub fn upsert_selected_field(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::UpsertField {
                handle,
                key: key.into(),
                value: value.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
    pub fn clear_selected_field(&mut self, key: impl Into<String>) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::ClearField {
                handle,
                key: key.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
}

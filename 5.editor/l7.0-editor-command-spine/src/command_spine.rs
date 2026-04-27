//! Command Spine — central command routing with validation, preconditions, and history.

pub use crate::command_types::*;

pub struct CommandSpine {
    command_history: Vec<ExecutedCommand>,
    undo_stack: Vec<CommandId>,
    redo_stack: Vec<CommandId>,
    has_project: bool,
    has_active_world: bool,
    has_selection: bool,
}

impl CommandSpine {
    pub fn new() -> Self {
        Self {
            command_history: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            has_project: false,
            has_active_world: false,
            has_selection: false,
        }
    }
    pub fn with_state(has_project: bool, has_active_world: bool, has_selection: bool) -> Self {
        Self {
            command_history: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            has_project,
            has_active_world,
            has_selection,
        }
    }
    pub fn set_has_project(&mut self, v: bool) {
        self.has_project = v;
    }
    pub fn set_has_active_world(&mut self, v: bool) {
        self.has_active_world = v;
    }
    pub fn set_has_selection(&mut self, v: bool) {
        self.has_selection = v;
    }

    pub fn validate_command(&self, command: &Command) -> Result<(), ValidationError> {
        if let CommandType::Custom(name) = &command.command_type {
            if name.is_empty() {
                return Err(ValidationError::new(
                    command.id,
                    "Custom command name cannot be empty",
                ));
            }
        }
        Ok(())
    }

    pub fn check_preconditions(&self, command: &Command) -> Result<(), PreconditionError> {
        let mut failed = Vec::new();
        match &command.command_type {
            CommandType::SaveProject | CommandType::LoadProject => {
                if !self.has_project {
                    failed.push("No project loaded".into());
                }
            }
            CommandType::SculptTerrain
            | CommandType::PaintTerrain
            | CommandType::ImportHeightmap => {
                if !self.has_project {
                    failed.push("No project loaded".into());
                }
                if !self.has_active_world {
                    failed.push("No active world".into());
                }
            }
            CommandType::CreateMaterial | CommandType::AssignTexture => {
                if !self.has_project {
                    failed.push("No project loaded".into());
                }
            }
            CommandType::TranslateEntity | CommandType::RotateEntity | CommandType::ScaleEntity => {
                if !self.has_selection {
                    failed.push("No entities selected".into());
                }
            }
            CommandType::SelectEntity | CommandType::DeselectAll | CommandType::Custom(_) => {}
        }
        if failed.is_empty() {
            Ok(())
        } else {
            Err(PreconditionError::new(command.id, failed))
        }
    }

    pub fn dispatch_command(&mut self, command: Command) -> Result<CommandResult, ExecutionError> {
        self.validate_command(&command).map_err(|e| {
            ExecutionError::new(
                command.id,
                format!("Validation failed: {}", e.reason),
                false,
            )
        })?;
        self.check_preconditions(&command).map_err(|e| {
            ExecutionError::new(
                command.id,
                format!("Preconditions failed: {:?}", e.failed_preconditions),
                false,
            )
        })?;
        let result = CommandResult::success(command.id);
        self.command_history.push(ExecutedCommand {
            command: command.clone(),
            result: result.clone(),
            undo_data: Some(UndoData::new()),
        });
        self.undo_stack.push(command.id);
        self.redo_stack.clear();
        Ok(result)
    }

    pub fn undo(&mut self) -> Result<(), UndoError> {
        let command_id = self
            .undo_stack
            .pop()
            .ok_or_else(|| UndoError::new(CommandId::new(), "No commands to undo"))?;
        self.redo_stack.push(command_id);
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), RedoError> {
        let command_id = self
            .redo_stack
            .pop()
            .ok_or_else(|| RedoError::new(CommandId::new(), "No commands to redo"))?;
        self.undo_stack.push(command_id);
        Ok(())
    }

    pub fn get_action_id(&self, command: &Command) -> ActionId {
        ActionId::from_command_type(&command.command_type)
    }
    pub fn command_history(&self) -> &[ExecutedCommand] {
        &self.command_history
    }
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for CommandSpine {
    fn default() -> Self {
        Self::new()
    }
}

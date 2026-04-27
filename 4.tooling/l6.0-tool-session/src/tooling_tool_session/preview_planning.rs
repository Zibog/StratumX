// Preview and Planning Operations

use super::runtime::ToolingRuntime;
use super::types::*;

impl ToolingRuntime {
    pub fn preview_object(&mut self, handle: ObjectHandle) -> Result<PreviewResult, ToolingError> {
        let object = self
            .objects
            .get(&handle)
            .ok_or(ToolingError::UnknownObject)?;
        Ok(PreviewResult {
            handle,
            summary: format!("preview:{:?}:{}", object.class, object.label),
        })
    }

    pub fn plan_goal(&self, goal: impl Into<String>) -> PlannedGoal {
        let goal = goal.into();
        PlannedGoal {
            goal: goal.clone(),
            suggested_commands: vec![format!("plan:{goal}")],
        }
    }

    pub fn stage_proposal(&mut self, _goal: impl Into<String>, _commands: Vec<String>) -> u64 {
        let proposal = self.next_proposal;
        self.next_proposal += 1;
        proposal
    }

    pub fn approve_proposal(&mut self, _proposal_id: u64) -> Result<(), ToolingError> {
        Ok(())
    }

    pub fn apply_proposal(&mut self, _proposal_id: u64) -> Result<(), ToolingError> {
        Ok(())
    }
}

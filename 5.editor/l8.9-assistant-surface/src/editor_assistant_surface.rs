//! FUTURE_STUB: this crate is intentionally not product-integrated yet.
//! It must not be counted as implemented editor functionality until wired into the active product spine.

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling_l6_0_tool_session::{PlannedGoal, ToolingError, ToolingRuntime};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AssistantSurface {
    pub goal_draft: String,
    pub staged_proposal: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    pub tooling: ToolingRuntime,
    pub assistant_surface: AssistantSurface,
}
impl EditorProduct {
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        Err(inactive_surface_error())
    }
    pub fn stage_assistant_goal(&mut self, goal: impl Into<String>) -> u64 {
        let goal = goal.into();
        let planned = self.tooling.plan_goal(goal.clone());
        let proposal_id = self
            .tooling
            .stage_proposal(goal, planned.suggested_commands);
        self.assistant_surface.staged_proposal = Some(proposal_id);
        proposal_id
    }
    pub fn approve_and_apply_assistant(&mut self) -> Result<(), ToolingError> {
        if let Some(proposal_id) = self.assistant_surface.staged_proposal {
            self.tooling.approve_proposal(proposal_id)?;
            self.tooling.apply_proposal(proposal_id)?;
            self.refresh_from_tooling()?;
        }
        Ok(())
    }
    pub fn planned_goal(&self, goal: impl Into<String>) -> PlannedGoal {
        self.tooling.plan_goal(goal)
    }
}

fn inactive_surface_error() -> ToolingError {
    ToolingError::Message(
        "FUTURE_STUB: assistant surface is not wired into the active editor product spine"
            .to_string(),
    )
}

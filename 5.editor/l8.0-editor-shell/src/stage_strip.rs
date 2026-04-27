//! Shell stage strip for navigating the proof lane.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceStage {
    World,
    Terrain,
    Environment,
    Simulation,
    Capture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageDefinition {
    pub stage: WorkspaceStage,
    pub id: &'static str,
    pub label: &'static str,
    pub panels: &'static [&'static str],
}

pub const STAGE_DEFINITIONS: &[StageDefinition] = &[
    StageDefinition {
        stage: WorkspaceStage::World,
        id: "stage.world",
        label: "World",
        panels: &["viewport", "outliner", "inspector"],
    },
    StageDefinition {
        stage: WorkspaceStage::Terrain,
        id: "stage.terrain",
        label: "Terrain",
        panels: &["viewport", "terrain", "inspector"],
    },
    StageDefinition {
        stage: WorkspaceStage::Environment,
        id: "stage.environment",
        label: "Environment",
        panels: &["viewport", "environment", "inspector"],
    },
    StageDefinition {
        stage: WorkspaceStage::Simulation,
        id: "stage.simulation",
        label: "Simulate",
        panels: &["viewport", "diagnostics"],
    },
    StageDefinition {
        stage: WorkspaceStage::Capture,
        id: "stage.capture",
        label: "Capture",
        panels: &["viewport", "diagnostics", "outliner"],
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageStrip {
    active_stage: WorkspaceStage,
}

impl StageStrip {
    pub fn new() -> Self {
        Self {
            active_stage: WorkspaceStage::World,
        }
    }

    pub fn active_stage(&self) -> WorkspaceStage {
        self.active_stage
    }

    pub fn definitions(&self) -> &'static [StageDefinition] {
        STAGE_DEFINITIONS
    }

    pub fn set_active(&mut self, stage: WorkspaceStage) {
        self.active_stage = stage;
    }

    pub fn set_active_by_id(&mut self, stage_id: &str) -> bool {
        if let Some(definition) = STAGE_DEFINITIONS
            .iter()
            .find(|definition| definition.id == stage_id)
        {
            self.active_stage = definition.stage;
            return true;
        }

        false
    }

    pub fn next(&mut self) {
        let index = STAGE_DEFINITIONS
            .iter()
            .position(|definition| definition.stage == self.active_stage)
            .unwrap_or(0);
        let next = (index + 1).min(STAGE_DEFINITIONS.len().saturating_sub(1));
        self.active_stage = STAGE_DEFINITIONS[next].stage;
    }

    pub fn previous(&mut self) {
        let index = STAGE_DEFINITIONS
            .iter()
            .position(|definition| definition.stage == self.active_stage)
            .unwrap_or(0);
        let previous = index.saturating_sub(1);
        self.active_stage = STAGE_DEFINITIONS[previous].stage;
    }

    pub fn panels_for_active_stage(&self) -> &'static [&'static str] {
        Self::panels_for(self.active_stage)
    }

    pub fn panels_for(stage: WorkspaceStage) -> &'static [&'static str] {
        STAGE_DEFINITIONS
            .iter()
            .find(|definition| definition.stage == stage)
            .map(|definition| definition.panels)
            .unwrap_or(&[])
    }
}

impl Default for StageStrip {
    fn default() -> Self {
        Self::new()
    }
}

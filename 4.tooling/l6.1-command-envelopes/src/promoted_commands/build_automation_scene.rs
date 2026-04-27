use serde::{Deserialize, Serialize};

/// Build and validation commands
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BuildCommand {
    Run,
    Release,
    ValidationRunFull,
    ValidationRunSmoke,
}

/// Automation commands
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AutomationCommand {
    RebuildAll,
    ValidateAll,
}

/// Scene commands (legacy vertical slice)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SceneCommand {
    Bootstrap,
    FireTestShot { weapon_entity_id: u32 },
    Reset,
}

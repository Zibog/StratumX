pub use runtime_inspector::*;

mod runtime_inspector;

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling::{
    ApprovalClass, BudgetClass, CommandOrigin, ObjectHandle, ToolCommand, ToolCommandResult,
    ToolingError, ToolingRuntime,
};
pub use stratumx_tooling_l6_0_tool_session as stratumx_tooling;

mod inspector_types;
mod label_and_fields;
mod tag_and_lifecycle;

#[cfg(feature = "desktop")]
pub mod inspector_panel;

pub use inspector_types::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorProduct {
    pub inspector: InspectorSystem,
    pub tooling: ToolingRuntime,
}
impl EditorProduct {
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        Ok(())
    }
}

impl Default for EditorProduct {
    fn default() -> Self {
        Self {
            inspector: InspectorSystem {
                selected: None,
                label: None,
                fields: Vec::new(),
            },
            tooling: ToolingRuntime::default(),
        }
    }
}

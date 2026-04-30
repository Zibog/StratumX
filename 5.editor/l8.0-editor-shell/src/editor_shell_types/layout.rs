//! Editor shell layout system types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkspaceLayoutSystem {
    pub open_panels: Vec<String>,
}

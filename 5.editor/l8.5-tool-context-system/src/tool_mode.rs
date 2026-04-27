//! Tool mode type.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolMode {
    Select,
    Move,
    Rotate,
    Scale,
    TerrainSculpt,
    Paint,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RouteDomain {
    Project,
    World,
    Import,
    Terrain,
    Material,
    Environment,
    Shell,
    Audio,
    Runtime,
    Build,
    Diagnostics,
}

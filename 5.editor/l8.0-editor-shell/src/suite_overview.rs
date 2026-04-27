use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuiteOverview {
    pub worlds: usize,
    pub scenes: usize,
    pub terrains: usize,
    pub materials: usize,
    pub logic_nodes: usize,
    pub assets: usize,
}

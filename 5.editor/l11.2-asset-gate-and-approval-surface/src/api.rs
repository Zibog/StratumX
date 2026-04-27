use crate::gate::*;
use std::collections::HashMap;

pub struct AssetGateService {
    gates: HashMap<uuid::Uuid, AssetGate>,
}

impl AssetGateService {
    pub fn new() -> Self {
        Self {
            gates: HashMap::new(),
        }
    }

    pub fn create_gate(&mut self, asset_id: uuid::Uuid) -> uuid::Uuid {
        let gate = AssetGate::new(asset_id);
        let id = gate.id;
        self.gates.insert(id, gate);
        id
    }
}

impl Default for AssetGateService {
    fn default() -> Self {
        Self::new()
    }
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trader {
    pub id: u64,
    pub position: [f32; 3],
    pub inventory_items: Vec<InventoryItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: u64,
    pub quantity: u32,
    pub price: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    pub id: u64,
    pub name: String,
    pub trigger_position: [f32; 3],
    pub trigger_radius: f32,
    pub completed: bool,
}

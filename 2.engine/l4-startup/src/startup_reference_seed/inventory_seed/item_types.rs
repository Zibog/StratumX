use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Ammo,
    Tool,
    Consumable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActorInventory {
    pub items: Vec<Item>,
    pub equipped_weapon: Option<u32>,
    pub max_slots: usize,
}

impl ActorInventory {
    pub fn new(max_slots: usize) -> Self {
        Self {
            items: Vec::new(),
            equipped_weapon: None,
            max_slots,
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_slots {
            return Err("Inventory full".to_string());
        }
        self.items.push(item);
        Ok(())
    }

    pub fn remove_item(&mut self, item_id: u32) -> Result<Item, String> {
        if let Some(pos) = self.items.iter().position(|i| i.id == item_id) {
            let item = self.items.remove(pos);
            if self.equipped_weapon == Some(item_id) {
                self.equipped_weapon = None;
            }
            Ok(item)
        } else {
            Err("Item not found in inventory".to_string())
        }
    }

    pub fn equip_weapon(&mut self, item_id: u32) -> Result<(), String> {
        if self
            .items
            .iter()
            .any(|i| i.id == item_id && i.item_type == ItemType::Weapon)
        {
            self.equipped_weapon = Some(item_id);
            Ok(())
        } else {
            Err("Weapon not found in inventory".to_string())
        }
    }

    pub fn unequip_weapon(&mut self) {
        self.equipped_weapon = None;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub position: [f32; 3],
    pub items: Vec<Item>,
    pub max_slots: usize,
}

impl Container {
    pub fn new(position: [f32; 3], max_slots: usize) -> Self {
        Self {
            position,
            items: Vec::new(),
            max_slots,
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_slots {
            return Err("Container full".to_string());
        }
        self.items.push(item);
        Ok(())
    }

    pub fn remove_item(&mut self, item_id: u32) -> Result<Item, String> {
        if let Some(pos) = self.items.iter().position(|i| i.id == item_id) {
            Ok(self.items.remove(pos))
        } else {
            Err("Item not found in container".to_string())
        }
    }
}

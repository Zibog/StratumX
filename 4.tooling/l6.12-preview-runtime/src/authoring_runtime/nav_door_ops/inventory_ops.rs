// Inventory Operations

use crate::VerticalSliceSession;
use engine_startup::startup_reference_seed::inventory_seed::{Item, ItemType};
use link_egress_observations::EditorAuthoringObservation;

pub fn handle_add_item(
    vs: &mut VerticalSliceSession,
    item_id: u32,
    item_name: String,
    item_type: String,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.add_item_to_inventory(Item {
        id: item_id,
        name: item_name.clone(),
        item_type: parse_item_type(&item_type)?,
    })?;
    Ok(EditorAuthoringObservation::ItemAddedToInventory { item_id, item_name })
}

pub fn handle_remove_item(
    vs: &mut VerticalSliceSession,
    item_id: u32,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.remove_item_from_inventory(item_id)?;
    Ok(EditorAuthoringObservation::ItemRemovedFromInventory { item_id })
}

pub fn handle_transfer_to_inventory(
    vs: &mut VerticalSliceSession,
    item_id: u32,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.transfer_item_container_to_inventory(item_id)?;
    Ok(EditorAuthoringObservation::ItemTransferredToInventory { item_id })
}

pub fn handle_transfer_to_container(
    vs: &mut VerticalSliceSession,
    item_id: u32,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.transfer_item_inventory_to_container(item_id)?;
    Ok(EditorAuthoringObservation::ItemTransferredToContainer { item_id })
}

pub fn handle_equip_weapon(
    vs: &mut VerticalSliceSession,
    item_id: u32,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.equip_weapon(item_id)?;
    Ok(EditorAuthoringObservation::WeaponEquipped { item_id })
}

pub fn handle_unequip_weapon(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.unequip_weapon();
    Ok(EditorAuthoringObservation::WeaponUnequipped)
}

pub fn handle_get_inventory_state(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let (items, equipped_weapon) = vs.runtime.get_inventory_state();
    Ok(EditorAuthoringObservation::InventoryStateInfo {
        items: items.into_iter().map(item_to_tuple).collect(),
        equipped_weapon,
    })
}

pub fn handle_get_container_state(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let items = vs.runtime.get_container_state();
    Ok(EditorAuthoringObservation::ContainerStateInfo {
        items: items.into_iter().map(item_to_tuple).collect(),
    })
}

// Helper functions

fn parse_item_type(item_type: &str) -> Result<ItemType, String> {
    match item_type {
        "Weapon" => Ok(ItemType::Weapon),
        "Ammo" => Ok(ItemType::Ammo),
        "Tool" => Ok(ItemType::Tool),
        "Consumable" => Ok(ItemType::Consumable),
        _ => Err(format!("Unknown item type: {item_type}")),
    }
}

fn item_to_tuple(item: Item) -> (u32, String, String) {
    (item.id, item.name, format!("{:?}", item.item_type))
}

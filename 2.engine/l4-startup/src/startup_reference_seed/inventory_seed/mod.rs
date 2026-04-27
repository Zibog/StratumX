// Inventory Seed

mod door_types;
mod item_types;
mod navigation_types;

pub use door_types::*;
pub use item_types::*;
pub use navigation_types::*;

pub fn create_default_door() -> DoorObject {
    DoorObject::new([0.0, 0.0, 5.0])
}

pub fn create_default_navigation_path() -> NavigationPath {
    NavigationPath::new([0.0, 0.0, 0.0], [0.0, 0.0, 10.0])
}

pub fn create_default_actor_inventory() -> ActorInventory {
    ActorInventory::new(10)
}

pub fn create_default_container() -> Container {
    let mut container = Container::new([2.0, 0.0, 2.0], 20);
    let weapon_item = Item {
        id: 1,
        name: "AK-47".to_string(),
        item_type: ItemType::Weapon,
    };
    container.add_item(weapon_item).ok();
    container
}

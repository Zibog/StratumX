use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavDoorInventoryCommand {
    OpenDoor,
    CloseDoor,
    SetDoorBlocked {
        reason: String,
    },
    SetDoorLocked,
    GetDoorState,
    SetNavigationPath {
        start: [f32; 3],
        destination: [f32; 3],
    },
    GetNavigationStatus,
    AddItemToInventory {
        item_id: u32,
        item_name: String,
        item_type: String,
    },
    RemoveItemFromInventory {
        item_id: u32,
    },
    TransferItemContainerToInventory {
        item_id: u32,
    },
    TransferItemInventoryToContainer {
        item_id: u32,
    },
    EquipWeapon {
        item_id: u32,
    },
    UnequipWeapon,
    GetInventoryState,
    GetContainerState,
    SaveProofSceneState,
    LoadProofSceneState {
        state_json: String,
    },
    ResetProofSceneBaseline,
    SaveFullWorldState,
    LoadFullWorldState {
        state_json: String,
    },
    GetWorldStateMetadata,
    RequestRegionLoad {
        region_key: (i32, i32, i32),
    },
    CompleteRegionLoad {
        region_key: (i32, i32, i32),
        size_bytes: usize,
    },
    RequestRegionUnload {
        region_key: (i32, i32, i32),
    },
    CompleteRegionUnload {
        region_key: (i32, i32, i32),
    },
    GetMemoryPressure,
    GetRegionResidency {
        region_key: (i32, i32, i32),
    },
    GetResidentRegions,
    GetMemoryUsage,
    WorldPosToRegion {
        position: [f32; 3],
    },
}

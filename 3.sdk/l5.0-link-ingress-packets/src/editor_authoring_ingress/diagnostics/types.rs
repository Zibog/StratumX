//! Diagnostic command types for editor authoring ingress
//!
//! Defines command enums for material, destruction, navigation, population, tactics, and ecology domains.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialArchetypeInput {
    pub label: String,
    pub mechanical_class: String,
    pub hardness_mohs: f32,
    pub brittleness: f32,
    pub density_kg_m3: f32,
    pub fracture_mode: String,
    pub penetration_resistance: f32,
    pub ricochet_bias: f32,
    pub debris_profile: String,
    pub dust_amount: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayerInput {
    pub archetype_id: u16,
    pub thickness_mm: f32,
    pub coverage: f32,
    pub bond_strength: f32,
    pub segmentation_mode: String,
    pub segment_size_mm: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialCommand {
    CreateArchetype {
        input: MaterialArchetypeInput,
    },
    ListArchetypes,
    CreateStack {
        label: String,
    },
    StackAddLayer {
        stack_id: u16,
        layer: MaterialLayerInput,
    },
    StackRemoveLayer {
        stack_id: u16,
        layer_index: u8,
    },
    AssignStackToEntitySlot {
        entity_id: u32,
        slot_id: u8,
        stack_id: u16,
    },
    GetStack {
        stack_id: u16,
    },
    ListStacks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialWorldCommand {
    SetBarrelWater { liters: f32 },
    GetBarrelWater,
    SetBarrelLeak { active: bool },
    IgniteFireObject,
    ExtinguishFireObject,
    SetFireObjectWetness { wetness_percent: f32 },
    GetFireObjectState,
    GetSmokeParticleCount,
    UpdateMaterialWorld { delta_time: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DestructionCommand {
    SetTerrainMaterial {
        material_type: String,
    },
    GetTerrainMaterial,
    TriggerBlast {
        position: [f32; 3],
        energy_j: f32,
    },
    GetTerrainBlastResponse,
    SetWallIntegrity {
        integrity: f32,
    },
    GetWallIntegrity,
    GetWallDestroyedState,
    SetSupportObjectType {
        structure_type: String,
    },
    GetSupportObjectState,
    ApplySupportDamage {
        energy_j: f32,
        impact_direction: [f32; 3],
    },
    GetDestructionSummary,
    ResetDestructionState,
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PopulationCommand {
    CreateNpcProfile {
        npc_id: u32,
        name: String,
        position: [f32; 3],
    },
    SetNpcTraits {
        aggression: f32,
        greed: f32,
        loyalty: f32,
        courage: f32,
        discipline: f32,
        sociability: f32,
    },
    GetNpcTraits,
    SetNpcNeed {
        need_type: String,
        value: f32,
    },
    GetNpcNeed {
        need_type: String,
    },
    SetNpcActivity {
        activity: String,
        start_time: f32,
        duration: f32,
        location: [f32; 3],
    },
    GetNpcSchedule,
    IncreaseScarcity {
        scarcity_factor: f32,
    },
    GetCrimePressure,
    EvaluateCrimeEscalation {
        scarcity_factor: f32,
    },
    GetCriminalStatus,
    SetNpcFaction {
        faction_id: u32,
        reputation: f32,
    },
    GetNpcFaction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TacticsCommand {
    CreateSquad {
        squad_id: u32,
        member_ids: Vec<u32>,
        roles: Vec<String>,
    },
    SetSquadMemberCover {
        npc_id: u32,
        cover_position: [f32; 3],
    },
    EvaluateSquadTactic {
        enemy_position: [f32; 3],
    },
    GetSquadTacticState,
    InvalidateSquadCover {
        destroyed_position: [f32; 3],
    },
    CheckCoverValidity {
        cover_position: [f32; 3],
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcologyCommand {
    CreateCreatureEcology {
        creature_id: u32,
        species: String,
        position: [f32; 3],
    },
    SetCreatureHunger {
        hunger: f32,
    },
    SetCreatureFear {
        fear: f32,
    },
    GetCreatureState,
    EvaluateCreatureMigration,
    GetCreatureMigration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReasonChainCommand {
    InspectNpc { npc_id: u32 },
    InspectScope,
    GetStats,
}

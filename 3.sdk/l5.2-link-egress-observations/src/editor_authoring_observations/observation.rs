//! Top-level editor authoring observation enum.

use crate::vertical_slice_observations::BallisticSimulationResultDto;
use serde::{Deserialize, Serialize};

use super::animation_observations::AnimationObservation;
use super::ballistics_observations::BallisticsObservation;
use super::diagnostics::{
    DiagnosticEntryDto, PreviewStateDto, ViewportStatsDto, WorldSummaryDto, WorldTreeNodeDto,
};
use super::environment::{SkyBundleStatusDto, StormFrontDto};
use super::material::{AuthoringMaterialStackDto, MaterialArchetypeDto};
use super::reason_chain::ReasonChainEntryDto;
use super::terrain::{AuthoringTerrainPatchDto, SurfaceRegionDto};
use super::world::{
    ActorDto, ActorPresetDto, AssetDto, AuthoringSceneDto, EntityDetailsDto, EntityDto,
};

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EditorAuthoringObservation {
    // Scene
    SceneCreated { scene: AuthoringSceneDto },
    EntityCreated { entity: EntityDto },
    EntityTransformUpdated { entity_id: u32 },
    EntityList { entities: Vec<EntityDto> },
    EntityDeleted { entity_id: u32 },
    EntityDetails { details: EntityDetailsDto },

    // Material
    MaterialArchetypeCreated { archetype: MaterialArchetypeDto },
    MaterialArchetypeList { archetypes: Vec<MaterialArchetypeDto> },
    MaterialStackCreated { stack: AuthoringMaterialStackDto },
    MaterialStackUpdated { stack: AuthoringMaterialStackDto },
    MaterialStackList { stacks: Vec<AuthoringMaterialStackDto> },
    MaterialStackDetails { stack: AuthoringMaterialStackDto },
    MaterialAssigned { entity_id: u32, slot_id: u8, stack_id: u16 },

    // Terrain
    TerrainPatchCreated { patch: AuthoringTerrainPatchDto },
    TerrainPatchList { patches: Vec<AuthoringTerrainPatchDto> },
    SurfacePainted { target_entity_id: u32, region: SurfaceRegionDto },
    TerrainPatchDetails { patch: AuthoringTerrainPatchDto },

    // Actor
    ActorSpawned { actor: ActorDto },
    ActorPresetList { presets: Vec<ActorPresetDto> },
    WeaponAttached { actor_id: u32, weapon_profile_id: u16 },
    ActiveActorSet { actor_id: u32 },
    ActiveActorInfo { actor: Option<ActorDto> },

    // Asset
    AssetImported { asset: AssetDto },
    AssetList { assets: Vec<AssetDto> },
    AssetDetails { asset: AssetDto },

    // Ballistics
    BallisticResult { result: BallisticSimulationResultDto },
    BallisticsEvent { event: BallisticsObservation },

    // World/Diagnostics
    WorldTree { nodes: Vec<WorldTreeNodeDto> },
    WorldSummary { summary: WorldSummaryDto },
    ViewportStats { stats: ViewportStatsDto },
    PreviewState { state: PreviewStateDto },
    Diagnostics { entries: Vec<DiagnosticEntryDto> },
    Error { message: String },

    // Material World
    BarrelWaterSet { liters: f32 },
    BarrelWaterInfo { liters: f32 },
    BarrelLeakSet { active: bool },
    RainSet { active: bool, intensity_mm_per_hour: f32 },
    WindSet { velocity: [f32; 3] },
    FireObjectIgnited { success: bool },
    FireObjectExtinguished,
    FireObjectWetnessSet { wetness_percent: f32 },
    FireObjectState { burning: bool, wetness_percent: f32, fuel_remaining_percent: f32 },
    SmokeParticleCount { count: usize },
    MaterialWorldUpdated { delta_time: f32 },

    // Destruction
    TerrainMaterialSet { material_type: String },
    TerrainMaterialInfo { material_type: String },
    BlastTriggered { position: [f32; 3], energy_j: f32 },
    TerrainBlastResponseInfo { crater_radius_m: f32, crater_depth_m: f32, debris_count: u32, ejecta_volume_m3: f32 },
    WallIntegritySet { integrity: f32 },
    WallIntegrityInfo { integrity: f32 },
    WallDestroyedStateInfo { destroyed: bool },
    SupportObjectTypeSet { structure_type: String },
    SupportObjectStateInfo { structure_type: String, integrity: f32, destroyed: bool, failure_mode: String, fragment_count: u32 },
    SupportDamageApplied { energy_j: f32 },
    DestructionSummaryInfo {
        terrain_material: String,
        crater_radius_m: Option<f32>,
        crater_depth_m: Option<f32>,
        crater_debris_count: Option<u32>,
        wall_integrity: f32,
        wall_destroyed: bool,
        support_structure_type: Option<String>,
        support_integrity: Option<f32>,
        support_destroyed: Option<bool>,
        support_failure_mode: Option<String>,
    },
    DestructionStateReset,

    // Nav/Door/Inventory
    DoorOpened,
    DoorClosed,
    DoorBlocked { reason: String },
    DoorLocked,
    DoorStateInfo { state: String, blocked_reason: Option<String> },
    NavigationPathSet { start: [f32; 3], destination: [f32; 3] },
    NavigationStatusInfo { status: String, blocked_reason: Option<String> },
    ItemAddedToInventory { item_id: u32, item_name: String },
    ItemRemovedFromInventory { item_id: u32 },
    ItemTransferredToInventory { item_id: u32 },
    ItemTransferredToContainer { item_id: u32 },
    WeaponEquipped { item_id: u32 },
    WeaponUnequipped,
    InventoryStateInfo { items: Vec<(u32, String, String)>, equipped_weapon: Option<u32> },
    ContainerStateInfo { items: Vec<(u32, String, String)> },
    ProofSceneStateSaved { state_json: String },
    ProofSceneStateLoaded,
    ProofSceneBaselineReset,
    FullWorldStateSaved { state_json: String, world_version: u32, save_timestamp: f64 },
    FullWorldStateLoaded { world_version: u32 },
    WorldStateMetadata { world_version: u32, simulation_time: f32, domains_count: usize },
    RegionLoadRequested { region_key: (i32, i32, i32) },
    RegionLoadCompleted { region_key: (i32, i32, i32), size_bytes: usize },
    RegionUnloadRequested { region_key: (i32, i32, i32) },
    RegionUnloadCompleted { region_key: (i32, i32, i32) },
    MemoryPressure { pressure: String, current_bytes: usize, budget_bytes: usize },
    RegionResidency { region_key: (i32, i32, i32), residency_state: String },
    ResidentRegions { regions: Vec<(i32, i32, i32)>, count: usize },
    MemoryUsage { current_bytes: usize, budget_bytes: usize, usage_percent: f32 },
    RegionKeyFromPosition { position: [f32; 3], region_key: (i32, i32, i32) },

    // Population/Tactics/Ecology
    NpcProfileCreated { npc_id: u32, name: String },
    NpcTraitsSet,
    NpcTraitsInfo { aggression: f32, greed: f32, loyalty: f32, courage: f32, discipline: f32, sociability: f32 },
    NpcNeedSet { need_type: String, value: f32 },
    NpcNeedInfo { need_type: String, value: f32 },
    NpcActivitySet { activity: String },
    NpcScheduleInfo { activity: String, start_time: f32, duration: f32, location: [f32; 3] },
    ScarcityIncreased { scarcity_factor: f32 },
    CrimePressureInfo { pressure: f32 },
    CrimeEscalationEvaluated { crime_type: Option<String> },
    CriminalStatusInfo { is_criminal: bool, reputation: f32, wanted_level: u8 },
    NpcFactionSet { faction_id: u32, reputation: f32 },
    NpcFactionInfo { faction_info: Option<(u32, f32, u8)> },
    SquadCreated { squad_id: u32, member_count: u32 },
    SquadMemberCoverSet { npc_id: u32, cover_position: [f32; 3] },
    SquadTacticEvaluated { reasons: Vec<String> },
    SquadTacticStateInfo { tactic_state: String },
    SquadCoverInvalidated { reasons: Vec<String> },
    CoverValidityInfo { valid: bool, reason: String },
    CreatureEcologyCreated { creature_id: u32, species: String },
    CreatureHungerSet { hunger: f32 },
    CreatureFearSet { fear: f32 },
    CreatureStateInfo { hunger: f32, fear: f32, migrating: bool },
    CreatureMigrationEvaluated { migration_info: Option<(String, [f32; 3])> },
    CreatureMigrationInfo { migration_state: Option<(String, [f32; 3], [f32; 3])> },

    // Sky/Weather
    SkySummaryRead {
        time_of_day_hours: f32,
        day_of_year: u16,
        latitude_deg: f32,
        sun_elevation_deg: f32,
        sun_intensity: f32,
        cloud_coverage: f32,
        fog_density: f32,
        rain_enabled: bool,
        rain_intensity_mm_per_hour: f32,
        wind_vector: [f32; 3],
        storm_front_count: usize,
    },
    SkyValueUpdated,
    SkySimulationStepped { time_of_day_hours: f32, sun_elevation_deg: f32 },
    StormFrontCreated { front_id: u32, position: [f32; 3], radius_km: f32 },
    StormFrontUpdated { front_id: u32 },
    StormFrontList { fronts: Vec<StormFrontDto> },
    SkyBundleStatusRead { status: SkyBundleStatusDto },
    SkyDiagnosticsRead {
        storm_front_count: usize,
        rain_enabled: bool,
        rain_intensity: f32,
        wind_magnitude: f32,
        fog_density: f32,
        cloud_coverage: f32,
    },
    SkyBaselineCaptured,
    SkyResetToBaseline,
    SkyRecoveredToDefault,

    // Reason Chain / Decision Trace
    ReasonChainNpcSlice { npc_id: u32, entries: Vec<ReasonChainEntryDto> },
    ReasonChainScopeSlice { entries: Vec<ReasonChainEntryDto> },
    ReasonTraceStats { event_count: usize },

    // Animation
    AnimationClipLoaded { clip_id: u32, duration: f32, frame_count: u32 },
    AnimationClipPlaying { entity_id: u32, clip_id: u32, current_time: f32, loop_count: u32 },
    AnimationClipStopped { entity_id: u32, clip_id: u32 },
    AnimationBlendUpdated { entity_id: u32, layer: u8, weight: f32 },
    IKTargetUpdated { entity_id: u32, ik_chain_id: u16, position: [f32; 3], weight: f32 },
    AnimationEventTriggered { entity_id: u32, event_name: String, timestamp: f32 },
    AnimationStateMachineCreated { entity_id: u32, state_count: u32 },
    AnimationError { entity_id: u32, error_message: String },
    AnimationObservation { observation: AnimationObservation },
}

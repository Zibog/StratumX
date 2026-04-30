use crate::{
    EntityDamageMemory, EntityMaterialBinding, ImpactRecord, MaterialWorldExecutor,
    ProofRegionScene, RuntimeEvent, ShotRecord, WorldApplyJournal, WorldCausalSummary,
    WorldManifest,
};
use engine_core::Tick;
use engine_ecs::EcsSubstrate;
use engine_world_region::RegionSubstrate;
use engine_world_spatial::{
    CellFrameRef, GeoAnchorRef, PrecisionZoneCode, RebaseDeltaRef, RegionFrameRef,
};

#[derive(Debug)]
pub struct WorldState {
    pub(crate) manifest: WorldManifest,
    pub(crate) ecs: EcsSubstrate,
    pub(crate) regions: RegionSubstrate,
    pub(crate) tick: Tick,
    pub(crate) epoch: u64,
    pub(crate) material_bindings: Vec<EntityMaterialBinding>,
    pub(crate) damage_memory: Vec<EntityDamageMemory>,
    pub(crate) shot_log: Vec<ShotRecord>,
    pub(crate) impact_log: Vec<ImpactRecord>,
    pub(crate) runtime_events: Vec<RuntimeEvent>,
    pub(crate) proof_region_scene: Option<ProofRegionScene>,
    pub(crate) material_world: MaterialWorldExecutor,
    pub(crate) last_apply_journal: WorldApplyJournal,
    pub(crate) last_causal_summary: WorldCausalSummary,
    pub(crate) geo_anchor_ref: GeoAnchorRef,
    pub(crate) active_region_frame_ref: Option<RegionFrameRef>,
    pub(crate) active_cell_frame_ref: Option<CellFrameRef>,
    pub(crate) last_rebase_delta_ref: Option<RebaseDeltaRef>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    pub fn new() -> Self {
        Self::new_with_manifest(WorldManifest::canonical_test_manifest())
            .expect("canonical test world manifest must validate")
    }

    pub fn new_with_manifest(manifest: WorldManifest) -> engine_core::EngineCoreResult<Self> {
        manifest.validate()?;
        Ok(Self {
            manifest,
            ecs: EcsSubstrate::new(),
            regions: RegionSubstrate::default(),
            tick: Tick(0),
            epoch: 0,
            material_bindings: Vec::new(),
            damage_memory: Vec::new(),
            shot_log: Vec::new(),
            impact_log: Vec::new(),
            runtime_events: Vec::new(),
            proof_region_scene: None,
            material_world: MaterialWorldExecutor::new(45.0),
            last_apply_journal: WorldApplyJournal {
                tick: Tick(0),
                epoch: 0,
                publish_passes: 0,
                segment_count: 0,
                segments: Vec::new(),
            },
            last_causal_summary: WorldCausalSummary {
                region_keys: Vec::new(),
                family_tags: Vec::new(),
                publish_passes: 0,
                near_region_frame_ref: None,
                far_phenomenon_track_refs: Vec::new(),
                precision_zone_code: PrecisionZoneCode::ReducedExact,
            },
            geo_anchor_ref: GeoAnchorRef::new([0, 0, 0]),
            active_region_frame_ref: None,
            active_cell_frame_ref: None,
            last_rebase_delta_ref: None,
        })
    }
}

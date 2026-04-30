use super::state::WorldState;
use crate::{
    EntityDamageMemory, EntityMaterialBinding, ImpactRecord, MaterialWorldExecutor,
    ProofRegionScene, ReadModel, RuntimeEvent, ShotRecord, WorldApplyJournal, WorldCausalSummary,
    WorldManifest,
};
use engine_core::Tick;
use engine_world_spatial::{CellFrameRef, GeoAnchorRef, RebaseDeltaRef, RegionFrameRef};

impl WorldState {
    pub fn material_world(&self) -> &MaterialWorldExecutor {
        &self.material_world
    }

    pub fn material_world_mut(&mut self) -> &mut MaterialWorldExecutor {
        &mut self.material_world
    }

    pub fn ecs_mut(&mut self) -> &mut engine_ecs::EcsSubstrate {
        &mut self.ecs
    }

    pub fn regions_mut(&mut self) -> &mut engine_world_region::RegionSubstrate {
        &mut self.regions
    }

    pub fn read_model(&self) -> ReadModel {
        ReadModel {
            tick: self.tick,
            epoch: self.epoch,
        }
    }

    pub fn set_proof_region_scene(&mut self, scene: ProofRegionScene) {
        self.proof_region_scene = Some(scene);
    }

    pub fn proof_region_scene(&self) -> Option<&ProofRegionScene> {
        self.proof_region_scene.as_ref()
    }

    pub fn proof_region_scene_mut(&mut self) -> Option<&mut ProofRegionScene> {
        self.proof_region_scene.as_mut()
    }

    pub fn add_material_binding(&mut self, binding: EntityMaterialBinding) {
        self.material_bindings.push(binding);
    }

    pub fn material_bindings(&self) -> &[EntityMaterialBinding] {
        &self.material_bindings
    }

    pub fn add_damage_memory(&mut self, memory: EntityDamageMemory) {
        self.damage_memory.push(memory);
    }

    pub fn damage_memory_mut(&mut self) -> &mut Vec<EntityDamageMemory> {
        &mut self.damage_memory
    }

    pub fn damage_memory(&self) -> &[EntityDamageMemory] {
        &self.damage_memory
    }

    pub fn log_shot(&mut self, record: ShotRecord) {
        self.shot_log.push(record);
    }

    pub fn shot_log(&self) -> &[ShotRecord] {
        &self.shot_log
    }

    pub fn log_impact(&mut self, record: ImpactRecord) {
        self.impact_log.push(record);
    }

    pub fn impact_log(&self) -> &[ImpactRecord] {
        &self.impact_log
    }

    pub fn emit_event(&mut self, event: RuntimeEvent) {
        self.runtime_events.push(event);
    }

    pub fn runtime_events(&self) -> &[RuntimeEvent] {
        &self.runtime_events
    }

    pub fn current_tick(&self) -> Tick {
        self.tick
    }

    pub fn manifest(&self) -> &WorldManifest {
        &self.manifest
    }

    pub fn replace_manifest(
        &mut self,
        manifest: WorldManifest,
    ) -> engine_core::EngineCoreResult<()> {
        manifest.validate()?;
        self.manifest = manifest;
        Ok(())
    }

    pub fn last_apply_journal(&self) -> &WorldApplyJournal {
        &self.last_apply_journal
    }

    pub fn causal_summary(&self) -> &WorldCausalSummary {
        &self.last_causal_summary
    }

    pub fn geo_anchor_ref(&self) -> GeoAnchorRef {
        self.geo_anchor_ref
    }

    pub fn active_region_frame_ref(&self) -> Option<RegionFrameRef> {
        self.active_region_frame_ref
    }

    pub fn active_cell_frame_ref(&self) -> Option<CellFrameRef> {
        self.active_cell_frame_ref
    }

    pub fn last_rebase_delta_ref(&self) -> Option<RebaseDeltaRef> {
        self.last_rebase_delta_ref
    }
}

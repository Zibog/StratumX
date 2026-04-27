use crate::{
    EntityDamageMemory, EntityMaterialBinding, ImpactRecord, MaterialWorldExecutor, ReadModel,
    RuntimeEvent, ShotRecord, VerticalSliceScene,
};
use engine_core::Tick;
use engine_ecs::EcsSubstrate;
use engine_world_region::RegionSubstrate;

#[derive(Debug)]
pub struct WorldState {
    pub(crate) ecs: EcsSubstrate,
    pub(crate) regions: RegionSubstrate,
    pub(crate) tick: Tick,
    pub(crate) epoch: u64,
    pub(crate) material_bindings: Vec<EntityMaterialBinding>,
    pub(crate) damage_memory: Vec<EntityDamageMemory>,
    pub(crate) shot_log: Vec<ShotRecord>,
    pub(crate) impact_log: Vec<ImpactRecord>,
    pub(crate) runtime_events: Vec<RuntimeEvent>,
    pub(crate) vertical_slice_scene: Option<VerticalSliceScene>,
    pub(crate) material_world: MaterialWorldExecutor,
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            ecs: EcsSubstrate::new(),
            regions: RegionSubstrate::default(),
            tick: Tick(0),
            epoch: 0,
            material_bindings: Vec::new(),
            damage_memory: Vec::new(),
            shot_log: Vec::new(),
            impact_log: Vec::new(),
            runtime_events: Vec::new(),
            vertical_slice_scene: None,
            material_world: MaterialWorldExecutor::new(45.0),
        }
    }

    pub fn material_world(&self) -> &MaterialWorldExecutor {
        &self.material_world
    }

    pub fn material_world_mut(&mut self) -> &mut MaterialWorldExecutor {
        &mut self.material_world
    }

    pub fn ecs_mut(&mut self) -> &mut EcsSubstrate {
        &mut self.ecs
    }

    pub fn regions_mut(&mut self) -> &mut RegionSubstrate {
        &mut self.regions
    }

    pub fn read_model(&self) -> ReadModel {
        ReadModel {
            tick: self.tick,
            epoch: self.epoch,
        }
    }

    pub fn set_vertical_slice_scene(&mut self, scene: VerticalSliceScene) {
        self.vertical_slice_scene = Some(scene);
    }

    pub fn vertical_slice_scene(&self) -> Option<&VerticalSliceScene> {
        self.vertical_slice_scene.as_ref()
    }

    pub fn vertical_slice_scene_mut(&mut self) -> Option<&mut VerticalSliceScene> {
        self.vertical_slice_scene.as_mut()
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
}

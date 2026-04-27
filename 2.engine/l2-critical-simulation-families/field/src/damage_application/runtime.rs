use super::policy::{apply_layer_damage_policy, DamagePolicyParams};
use super::types::{
    DamageApplicationResult, LayerImpactInput, MAX_DEBRIS_PROXIES_PER_SHOT,
    MAX_DUST_EVENTS_PER_SHOT, MAX_RELEASED_SEGMENTS_PER_SHOT,
};
use engine_core::{EngineCoreError, EngineCoreResult};
use engine_material::MaterialRegistry;
use engine_world::{EntityId, RuntimeEvent, WorldState};

pub struct DamageApplicator {
    released_segment_budget: usize,
    debris_budget: usize,
    dust_budget: usize,
}

impl DamageApplicator {
    pub fn new() -> Self {
        Self {
            released_segment_budget: MAX_RELEASED_SEGMENTS_PER_SHOT,
            debris_budget: MAX_DEBRIS_PROXIES_PER_SHOT,
            dust_budget: MAX_DUST_EVENTS_PER_SHOT,
        }
    }

    pub fn apply_impact_to_world(
        &mut self,
        world: &mut WorldState,
        materials: &MaterialRegistry,
        target_entity: EntityId,
        layer_impacts: &[LayerImpactInput],
    ) -> EngineCoreResult<DamageApplicationResult> {
        let tick = world.current_tick();

        let mut total_cracked = Vec::new();
        let mut total_released = Vec::new();
        let mut total_debris = 0;
        let mut total_dust = 0;

        let material_stack_id = {
            let damage_memory = world
                .damage_memory_mut()
                .iter()
                .find(|m| m.entity_id == target_entity)
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "damage memory not found for entity",
                ))?;
            engine_material::MaterialStackId(damage_memory.stack_id.0)
        };

        let stack =
            materials
                .stack(material_stack_id)
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "material stack not found",
                ))?;

        for impact in layer_impacts {
            let damage_memory = world
                .damage_memory_mut()
                .iter_mut()
                .find(|m| m.entity_id == target_entity)
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "damage memory not found for entity",
                ))?;

            let layer_state = damage_memory
                .layer_damage
                .iter_mut()
                .find(|l| l.layer_index == impact.layer_index)
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "layer damage state not found",
                ))?;

            let layer_def = stack
                .layers
                .get(impact.layer_index as usize)
                .ok_or(EngineCoreError::InvalidDescriptor("layer not found"))?;

            let archetype = materials.archetype(layer_def.archetype_id).ok_or(
                EngineCoreError::InvalidDescriptor("material archetype not found"),
            )?;

            layer_state.accumulated_energy_j += impact.energy_absorbed_j;

            let result = apply_layer_damage_policy(DamagePolicyParams {
                tick,
                entity: target_entity,
                layer_state,
                archetype,
                impact,
                released_segment_budget: &mut self.released_segment_budget,
                debris_budget: &mut self.debris_budget,
                dust_budget: &mut self.dust_budget,
            })?;

            total_cracked.extend(&result.cracked_segments);
            total_released.extend(&result.released_segments);
            total_debris += result.debris_count;
            total_dust += result.dust_events;

            for seg_id in &result.cracked_segments {
                world.emit_event(RuntimeEvent::SegmentCracked {
                    tick,
                    entity: target_entity,
                    layer: impact.layer_index,
                    segment: *seg_id,
                });
            }

            for seg_id in &result.released_segments {
                world.emit_event(RuntimeEvent::SegmentReleased {
                    tick,
                    entity: target_entity,
                    layer: impact.layer_index,
                    segment: *seg_id,
                });
            }
        }

        Ok(DamageApplicationResult {
            cracked_segments: total_cracked,
            released_segments: total_released,
            debris_count: total_debris,
            dust_events: total_dust,
        })
    }
}

impl Default for DamageApplicator {
    fn default() -> Self {
        Self::new()
    }
}

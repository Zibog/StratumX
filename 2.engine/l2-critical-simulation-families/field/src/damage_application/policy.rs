use super::types::{DamageApplicationResult, LayerImpactInput};
use engine_core::{EngineCoreResult, Tick};
use engine_material::{FractureMode, MaterialArchetype, SegmentationMode};
use engine_world::{EntityId, LayerDamageState};

pub struct DamagePolicyParams<'a> {
    pub tick: Tick,
    pub entity: EntityId,
    pub layer_state: &'a mut LayerDamageState,
    pub archetype: &'a MaterialArchetype,
    pub impact: &'a LayerImpactInput,
    pub released_segment_budget: &'a mut usize,
    pub debris_budget: &'a mut usize,
    pub dust_budget: &'a mut usize,
}

pub fn apply_layer_damage_policy(
    params: DamagePolicyParams<'_>,
) -> EngineCoreResult<DamageApplicationResult> {
    let tick = params.tick;
    let entity = params.entity;
    let layer_state = params.layer_state;
    let archetype = params.archetype;
    let impact = params.impact;
    let released_segment_budget = params.released_segment_budget;
    let debris_budget = params.debris_budget;
    let dust_budget = params.dust_budget;
    let mut cracked = Vec::new();
    let mut released = Vec::new();
    let mut debris_count = 0;
    let mut dust_events = 0;

    let energy_threshold = archetype.fracture_energy_j_m2;

    if layer_state.accumulated_energy_j > energy_threshold {
        match archetype.fracture_mode {
            FractureMode::Segment => {
                let (c, r) = apply_segment_fracture(
                    tick,
                    entity,
                    layer_state,
                    archetype,
                    impact,
                    released_segment_budget,
                )?;
                cracked = c;
                released = r;
            }
            FractureMode::Crumble => {
                dust_events = apply_crumble(tick, entity, layer_state.layer_index, dust_budget);
            }
            FractureMode::Chip => {
                debris_count = apply_chip(tick, entity, layer_state.layer_index, debris_budget);
            }
            _ => {}
        }

        let damage_ratio = layer_state.accumulated_energy_j / (energy_threshold * 2.0);
        layer_state.integrity = (1.0 - damage_ratio).max(0.0);
    }

    Ok(DamageApplicationResult {
        cracked_segments: cracked,
        released_segments: released,
        debris_count,
        dust_events,
    })
}

fn apply_segment_fracture(
    _tick: Tick,
    _entity: EntityId,
    layer_state: &mut LayerDamageState,
    archetype: &MaterialArchetype,
    _impact: &LayerImpactInput,
    released_segment_budget: &mut usize,
) -> EngineCoreResult<(Vec<u16>, Vec<u16>)> {
    let mut cracked = Vec::new();
    let mut released = Vec::new();

    let segment_count = match archetype.segmentation {
        SegmentationMode::Grid { rows, cols } => (rows as usize) * (cols as usize),
        SegmentationMode::Voronoi { seed_count } => seed_count as usize,
        SegmentationMode::None => 0,
    };

    if segment_count == 0 {
        return Ok((cracked, released));
    }

    if layer_state.cracked_segments.is_empty() && layer_state.released_segments.is_empty() {
        layer_state.cracked_segments = vec![];
        layer_state.released_segments = vec![];
    }

    let crack_count = (segment_count / 4).max(1);
    for i in 0..crack_count {
        let seg_id = i as u16;
        if !layer_state.cracked_segments.contains(&seg_id)
            && !layer_state.released_segments.contains(&seg_id)
        {
            layer_state.cracked_segments.push(seg_id);
            cracked.push(seg_id);
        }
    }

    let release_count = (crack_count / 2).max(1).min(*released_segment_budget);
    for i in 0..release_count {
        let seg_id = i as u16;
        if layer_state.cracked_segments.contains(&seg_id)
            && !layer_state.released_segments.contains(&seg_id)
        {
            layer_state.released_segments.push(seg_id);
            released.push(seg_id);
            *released_segment_budget -= 1;
        }
    }

    Ok((cracked, released))
}

fn apply_crumble(
    _tick: Tick,
    _entity: EntityId,
    _layer_index: u8,
    dust_budget: &mut usize,
) -> usize {
    let dust = 1.min(*dust_budget);
    *dust_budget -= dust;
    dust
}

fn apply_chip(
    _tick: Tick,
    _entity: EntityId,
    _layer_index: u8,
    debris_budget: &mut usize,
) -> usize {
    let debris = 1.min(*debris_budget);
    *debris_budget -= debris;
    debris
}

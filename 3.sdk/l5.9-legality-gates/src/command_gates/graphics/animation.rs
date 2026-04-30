//! Animation command legality gates.

use crate::command_gates::common::{
    invalid, max_len, non_empty, non_negative, non_zero_u16, non_zero_u32, normalized, positive,
    GateResult,
};
use crate::command_gates::verdict::LegalityVerdict;

pub fn validate_animation_play_clip(entity_id: u32, clip_id: u32, blend_time: f32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    non_zero_u32(clip_id, "clip_id", "Clip ID")?;
    non_negative(blend_time, "blend_time", "Blend time")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_stop_clip(entity_id: u32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_set_speed(entity_id: u32, speed: f32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    positive(speed, "speed", "Animation speed")?;
    if speed > 100.0 {
        return Err(invalid(
            "speed",
            format!("Animation speed excessively high: {speed} > 100"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_set_weight(entity_id: u32, layer: u8, weight: f32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    normalized(weight, "weight", "Blend weight")?;
    let _ = layer;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_set_ik_target(
    entity_id: u32,
    ik_chain_id: u16,
    position: [f32; 3],
    rotation: [f32; 4],
    weight: f32,
) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    non_zero_u16(ik_chain_id, "ik_chain_id", "IK chain ID")?;
    normalized(weight, "weight", "IK weight")?;
    let rot_magnitude_sq = rotation.iter().map(|value| value * value).sum::<f32>();
    if !(0.0001..=10.0).contains(&rot_magnitude_sq) {
        return Err(invalid(
            "rotation",
            "Rotation quaternion has invalid magnitude",
        ));
    }
    let _ = position;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_load_clip(clip_id: u32, clip_data: &[u8]) -> GateResult {
    non_zero_u32(clip_id, "clip_id", "Clip ID")?;
    if clip_data.is_empty() {
        return Err(invalid("clip_data", "Clip data cannot be empty"));
    }
    if clip_data.len() > 100_000_000 {
        return Err(invalid(
            "clip_data",
            format!("Clip data too large: {} > 100MB", clip_data.len()),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_create_state_machine(
    entity_id: u32,
    states: &[(u16, String, u32)],
    transitions: &[(u16, u16, String, f32)],
) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    if states.is_empty() {
        return Err(invalid(
            "states",
            "State machine must have at least one state",
        ));
    }
    if states.len() > 1000 {
        return Err(invalid(
            "states",
            format!("State machine has too many states: {} > 1000", states.len()),
        ));
    }
    for (index, (_, label, _)) in states.iter().enumerate() {
        if label.is_empty() {
            return Err(invalid(
                "state_label",
                format!("State label at index {index} cannot be empty"),
            ));
        }
    }
    for (index, (from, to, trigger, blend)) in transitions.iter().enumerate() {
        if *from == 0 || *to == 0 {
            return Err(invalid(
                "transition_states",
                format!("Transition at index {index} has invalid state ID (from={from}, to={to})"),
            ));
        }
        if trigger.is_empty() {
            return Err(invalid(
                "transition_trigger",
                format!("Transition trigger at index {index} cannot be empty"),
            ));
        }
        if *blend < 0.0 {
            return Err(invalid(
                "blend_duration",
                format!(
                    "Transition blend duration at index {index} cannot be negative, got {blend}"
                ),
            ));
        }
    }
    let _ = (states, transitions);
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_trigger_event(entity_id: u32, event_name: &str) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    non_empty(event_name, "event_name", "Event name cannot be empty")?;
    max_len(event_name, 256, "event_name", "Event name")?;
    Ok(LegalityVerdict::Legal)
}

//! Asset and streaming command legality gates.
//!
//! This module contains asset import/detail checks plus world-state and region
//! load validations carried by the legacy command gate surface.

use super::common::{invalid, non_empty, non_zero_u32, non_zero_usize, GateResult};
use super::verdict::LegalityVerdict;

pub fn validate_asset_import(path: &str, label: &str) -> GateResult {
    non_empty(path, "path", "Asset path cannot be empty")?;
    non_empty(label, "label", "Asset label cannot be empty")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_asset_get_details(asset_id: u32) -> GateResult {
    non_zero_u32(asset_id, "asset_id", "Asset ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_world_load_state(state_json: &str) -> GateResult {
    non_empty(state_json, "state_json", "State JSON cannot be empty")?;
    if state_json.len() > 10_000_000 {
        return Err(invalid(
            "state_json",
            format!("State JSON too large: {} > 10MB", state_json.len()),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_region_request(region_key: (i32, i32, i32)) -> GateResult {
    let _ = region_key;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_region_complete_load(region_key: (i32, i32, i32), size_bytes: usize) -> GateResult {
    non_zero_usize(size_bytes, "size_bytes", "Region size")?;
    let _ = region_key;
    Ok(LegalityVerdict::Legal)
}

use crate::model::{BranchCoverage, MaterialProfile};

pub fn inspect_branch_coverage(profile: &MaterialProfile) -> BranchCoverage {
    let mut missing = Vec::new();
    let mut invalid = Vec::new();

    let physical_complete = profile.material_archetype_ref.is_some()
        && profile.surface_family_ref.is_some()
        && profile.response_profile_ref.is_some()
        && profile.physical_response_family.is_some();

    let visual_complete = profile.visual_response.is_some();
    let acoustic_complete = profile.acoustic_profile.is_some();
    let light_complete = profile.light_response.is_some();
    let runtime_complete = profile.cheap_runtime_rung.is_configured();
    let persistence_complete = profile.persistence_family.is_some();
    let proof_complete = profile.proof_family.is_some();

    if profile.material_archetype_ref.is_none() {
        missing.push("material_archetype_ref".to_string());
    }
    if profile.surface_family_ref.is_none() {
        missing.push("surface_family_ref".to_string());
    }
    if profile.response_profile_ref.is_none() {
        missing.push("response_profile_ref".to_string());
    }
    if profile.physical_response_family.is_none() {
        missing.push("physical_response_family".to_string());
    }
    if !visual_complete {
        missing.push("visual_response".to_string());
    }
    if !acoustic_complete {
        missing.push("acoustic_profile".to_string());
    }
    if !light_complete {
        missing.push("light_response".to_string());
    }
    if !runtime_complete {
        missing.push("cheap_runtime_rung".to_string());
    }
    if !persistence_complete {
        missing.push("persistence_family".to_string());
    }
    if !proof_complete {
        missing.push("proof_family".to_string());
    }

    if profile.weather_modulation.is_some() && !visual_complete {
        invalid.push("weather_modulation requires visual_response".to_string());
    }

    BranchCoverage {
        physical_complete,
        visual_complete,
        acoustic_complete,
        light_complete,
        runtime_complete,
        persistence_complete,
        proof_complete,
        missing_bindings: missing,
        invalid_combinations: invalid,
    }
}

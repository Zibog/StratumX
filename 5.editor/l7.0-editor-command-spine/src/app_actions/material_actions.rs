// Material authoring actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn create_material(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|material_name| PromotedCommand::MaterialCreate { material_name })
}

pub fn delete_material(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|material_id| PromotedCommand::MaterialDelete { material_id })
}

pub fn bind_visual_response(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::Multiple(parts)) = payload {
        if parts.len() >= 2 {
            if let (Some(material_id), Some(visual_family)) = (
                parts[0].as_string().map(|s| s.to_string()),
                parts[1].as_string().map(|s| s.to_string()),
            ) {
                return Some(PromotedCommand::MaterialBindVisualResponse {
                    material_id,
                    visual_family,
                });
            }
        }
    }
    None
}

pub fn bind_acoustic_profile(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::Multiple(parts)) = payload {
        if parts.len() >= 2 {
            if let (Some(material_id), Some(acoustic_profile)) = (
                parts[0].as_string().map(|s| s.to_string()),
                parts[1].as_string().map(|s| s.to_string()),
            ) {
                return Some(PromotedCommand::MaterialBindAcousticProfile {
                    material_id,
                    acoustic_profile,
                });
            }
        }
    }
    None
}

pub fn bind_light_response(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::Multiple(parts)) = payload {
        if parts.len() >= 2 {
            if let (Some(material_id), Some(light_response)) = (
                parts[0].as_string().map(|s| s.to_string()),
                parts[1].as_string().map(|s| s.to_string()),
            ) {
                return Some(PromotedCommand::MaterialBindLightResponse {
                    material_id,
                    light_response,
                });
            }
        }
    }
    None
}

pub fn set_cheap_runtime_rung(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::Multiple(parts)) = payload {
        if parts.len() >= 2 {
            if let (Some(material_id), Some(rung_level)) = (
                parts[0].as_string().map(|s| s.to_string()),
                parts[1].as_u32(),
            ) {
                return Some(PromotedCommand::MaterialSetCheapRuntimeRung {
                    material_id,
                    rung_level,
                });
            }
        }
    }
    None
}

pub fn inspect_branch_coverage(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|material_id| PromotedCommand::MaterialInspectBranchCoverage { material_id })
}

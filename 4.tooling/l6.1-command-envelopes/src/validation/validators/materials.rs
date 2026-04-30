// Material command validators

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;
use crate::validation::rules::*;

pub fn validate_material_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        PromotedCommand::MaterialAuthorityInitialize
        | PromotedCommand::MaterialAuthorityDispose => ValidationResult::Valid,
        PromotedCommand::MaterialCreate { material_name } => {
            validate_non_empty(material_name, "material_name")
        }
        PromotedCommand::MaterialDelete { material_id } => {
            validate_non_empty(material_id, "material_id")
        }
        PromotedCommand::MaterialDuplicateProfile {
            source_material_id,
            target_name,
        } => validate_material_duplicate(source_material_id, target_name),
        PromotedCommand::MaterialBindVisualResponse {
            material_id,
            visual_family,
        } => validate_material_binding(material_id, visual_family),
        PromotedCommand::MaterialBindAcousticProfile {
            material_id,
            acoustic_profile,
        } => validate_material_binding(material_id, acoustic_profile),
        PromotedCommand::MaterialBindLightResponse {
            material_id,
            light_response,
        } => validate_material_binding(material_id, light_response),
        PromotedCommand::MaterialBindMicrodetailProfile {
            material_id,
            microdetail_profile,
        } => validate_material_binding(material_id, microdetail_profile),
        PromotedCommand::MaterialBindWeatherModulation {
            material_id,
            weather_modulation,
        } => validate_material_binding(material_id, weather_modulation),
        PromotedCommand::MaterialPreviewBurn {
            material_id,
            preview_target,
        } => validate_material_binding(material_id, preview_target),
        PromotedCommand::MaterialSetCheapRuntimeRung {
            material_id,
            rung_level,
        } => validate_material_rung(material_id, *rung_level),
        PromotedCommand::MaterialInspectBranchCoverage { material_id } => {
            validate_non_empty(material_id, "material_id")
        }
        _ => ValidationResult::Valid,
    }
}

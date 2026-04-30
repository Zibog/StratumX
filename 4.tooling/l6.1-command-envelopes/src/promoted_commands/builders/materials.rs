// Material command builders

use crate::promoted_commands::types::PromotedCommand;

/// Builder for material commands
pub struct MaterialCommandBuilder;

impl MaterialCommandBuilder {
    pub fn create(material_name: impl Into<String>) -> PromotedCommand {
        PromotedCommand::MaterialCreate {
            material_name: material_name.into(),
        }
    }

    pub fn delete(material_id: impl Into<String>) -> PromotedCommand {
        PromotedCommand::MaterialDelete {
            material_id: material_id.into(),
        }
    }

    pub fn bind_visual_response(
        material_id: impl Into<String>,
        visual_family: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::MaterialBindVisualResponse {
            material_id: material_id.into(),
            visual_family: visual_family.into(),
        }
    }

    pub fn bind_acoustic_profile(
        material_id: impl Into<String>,
        acoustic_profile: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::MaterialBindAcousticProfile {
            material_id: material_id.into(),
            acoustic_profile: acoustic_profile.into(),
        }
    }
}

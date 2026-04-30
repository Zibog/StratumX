use crate::material_response::{
    ConsequenceTier, MaterialStateModifier, MaterialTriggerClass, ResponseFamilyGroup,
};
use crate::{AftermathFamilyId, ResponseFamilyId, SurfaceFamilyId};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialConsequenceSelection {
    pub surface_family_id: SurfaceFamilyId,
    pub response_family_id: ResponseFamilyId,
    pub response_family_group: ResponseFamilyGroup,
    pub trigger_class: MaterialTriggerClass,
    pub previous_tier: ConsequenceTier,
    pub next_tier: ConsequenceTier,
    pub persistence_required: bool,
    pub state_modifiers: Vec<MaterialStateModifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialConsequenceEvent {
    pub surface_family_id: String,
    pub response_family_id: String,
    pub response_family_group: ResponseFamilyGroup,
    pub trigger_class: MaterialTriggerClass,
    pub previous_tier: ConsequenceTier,
    pub next_tier: ConsequenceTier,
    pub downstream_truth_required: bool,
    pub persistence_required: bool,
    pub state_modifiers: Vec<MaterialStateModifier>,
}

impl MaterialConsequenceEvent {
    pub fn from_response_selection(selection: MaterialConsequenceSelection) -> Self {
        Self {
            surface_family_id: selection.surface_family_id.into_string(),
            response_family_id: selection.response_family_id.into_string(),
            response_family_group: selection.response_family_group,
            trigger_class: selection.trigger_class,
            previous_tier: selection.previous_tier,
            next_tier: selection.next_tier,
            downstream_truth_required: selection.next_tier >= ConsequenceTier::DownstreamConsequence,
            persistence_required: selection.persistence_required,
            state_modifiers: selection.state_modifiers,
        }
    }

    pub fn from_burn(
        surface_family_id: SurfaceFamilyId,
        response_family_id: ResponseFamilyId,
        next_tier: ConsequenceTier,
        downstream_truth_required: bool,
        state_modifiers: Vec<MaterialStateModifier>,
        aftermath_family_id: AftermathFamilyId,
    ) -> Self {
        Self {
            surface_family_id: surface_family_id.into_string(),
            response_family_id: response_family_id.into_string(),
            response_family_group: ResponseFamilyGroup::PhysicalThermal,
            trigger_class: MaterialTriggerClass::FireExposure,
            previous_tier: ConsequenceTier::Dormant,
            next_tier,
            downstream_truth_required,
            persistence_required: aftermath_family_id.requires_persistence(),
            state_modifiers,
        }
    }

    pub fn surface_family_key(&self) -> SurfaceFamilyId {
        SurfaceFamilyId::from(self.surface_family_id.clone())
    }

    pub fn response_family_key(&self) -> ResponseFamilyId {
        ResponseFamilyId::from(self.response_family_id.clone())
    }
}

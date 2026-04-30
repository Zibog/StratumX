use super::{BurnMaterialPublicationPolicy, MaterialConsequenceEvent};
use crate::material_response::ConsequenceTier;
use crate::{
    BurnAftermathPolicy, BurnConsequenceReceipt, BurnResponseFamily, MaterialId,
    MaterialStateModifier, PersistentBurnResult, ResponseFamilyId, SurfaceFamilyId,
};
use engine_core::StableDigestBuilder;

impl BurnConsequenceReceipt {
    pub fn from_runtime(
        material_id: MaterialId,
        burn_family: BurnResponseFamily,
        ignited: bool,
        fuel_consumed_percent: f32,
        wetness_percent: f32,
        aftermath_policy: BurnAftermathPolicy,
        persistent_result: PersistentBurnResult,
    ) -> Self {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.material.burn")
            .write_u16(material_id.0)
            .write_u8(burn_family as u8)
            .write_bool(ignited)
            .write_u32(fuel_consumed_percent.to_bits())
            .write_u32(wetness_percent.to_bits())
            .write_u8(aftermath_policy as u8)
            .write_u8(persistent_result as u8);
        Self {
            material_id,
            burn_family,
            ignited,
            fuel_consumed_percent,
            wetness_percent,
            aftermath_policy,
            persistent_result,
            deterministic_digest: digest.finish().0,
        }
    }

    pub fn to_material_consequence_event(&self) -> MaterialConsequenceEvent {
        let publication = self.publication_policy();
        MaterialConsequenceEvent::from_burn(
            publication,
            self.next_tier(),
            self.ignited,
            self.state_modifiers(),
        )
    }

    pub fn publication_policy(&self) -> BurnMaterialPublicationPolicy {
        BurnMaterialPublicationPolicy {
            surface_family: SurfaceFamilyId::from_material_id(self.material_id),
            response_family: ResponseFamilyId::physical_thermal(self.burn_family.response_suffix()),
            aftermath_family: self.persistent_result.aftermath_family_id(),
        }
    }

    fn state_modifiers(&self) -> Vec<MaterialStateModifier> {
        match self.persistent_result {
            PersistentBurnResult::Intact if self.wetness_percent >= 20.0 => {
                vec![MaterialStateModifier::Wet]
            }
            PersistentBurnResult::Intact => vec![MaterialStateModifier::Clean],
            PersistentBurnResult::Charred => vec![MaterialStateModifier::Charred],
            PersistentBurnResult::Ash => {
                vec![MaterialStateModifier::Charred, MaterialStateModifier::Dusty]
            }
            PersistentBurnResult::Removed => {
                vec![
                    MaterialStateModifier::Charred,
                    MaterialStateModifier::Fractured,
                ]
            }
        }
    }

    fn next_tier(&self) -> ConsequenceTier {
        match self.persistent_result {
            PersistentBurnResult::Intact if self.ignited => ConsequenceTier::LocalConsequence,
            PersistentBurnResult::Intact if self.wetness_percent > 0.0 => {
                ConsequenceTier::LocalConsequence
            }
            PersistentBurnResult::Intact => ConsequenceTier::Dormant,
            PersistentBurnResult::Charred => ConsequenceTier::DownstreamConsequence,
            PersistentBurnResult::Ash | PersistentBurnResult::Removed => ConsequenceTier::FarEcho,
        }
    }
}

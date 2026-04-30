use super::{MaterialConsequenceEvent, MaterialConsequenceSelection};
use crate::material_response::{
    ConsequenceTier, MaterialInstanceProfile, MaterialResponseProfile, MaterialStateModifier,
    MaterialTriggerClass, ResponseFamilyRow, SurfaceFamilyProfile,
};
use crate::{MaterialRegistry, ResponseFamilyId, SurfaceFamilyId};
use engine_core::{EngineCoreError, EngineCoreResult};

impl MaterialRegistry {
    pub fn register_surface_family(
        &mut self,
        profile: SurfaceFamilyProfile,
    ) -> EngineCoreResult<()> {
        profile.validate()?;
        if !self.archetypes.contains_key(&profile.primary_archetype_ref) {
            return Err(EngineCoreError::InvalidDescriptor(
                "surface family references unregistered archetype",
            ));
        }
        self.surface_families.insert(
            SurfaceFamilyId::from(profile.surface_family_id.clone()),
            profile,
        );
        Ok(())
    }

    pub fn register_response_family_row(&mut self, row: ResponseFamilyRow) -> EngineCoreResult<()> {
        row.validate()?;
        self.response_family_rows
            .insert(ResponseFamilyId::from(row.family_id.clone()), row);
        Ok(())
    }

    pub fn register_response_profile(
        &mut self,
        profile: MaterialResponseProfile,
    ) -> EngineCoreResult<()> {
        profile.validate()?;
        for family_id in [
            &profile.contact_response_family,
            &profile.penetration_response_family,
            &profile.blast_response_family,
            &profile.burn_response_family,
            &profile.wetness_response_family,
            &profile.fracture_response_family,
            &profile.traversal_response_family,
            &profile.persistence_response_family,
        ] {
            if !self.response_family_rows.contains_key(family_id.as_str()) {
                return Err(EngineCoreError::InvalidDescriptor(
                    "response profile references unregistered family row",
                ));
            }
        }
        self.response_profiles
            .insert(profile.response_profile_id, profile);
        Ok(())
    }

    pub fn register_instance_profile(
        &mut self,
        profile: MaterialInstanceProfile,
    ) -> EngineCoreResult<()> {
        profile.validate()?;
        let stack =
            self.stacks
                .get(&profile.stack_id)
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "instance profile references unregistered stack",
                ))?;
        let surface = self
            .surface_families
            .get(profile.surface_family_id.as_str())
            .ok_or(EngineCoreError::InvalidDescriptor(
                "instance profile references unregistered surface",
            ))?;
        if surface.territory_family != profile.territory_family {
            return Err(EngineCoreError::InvalidDescriptor(
                "instance profile territory must match surface family territory",
            ));
        }
        if surface.response_profile_ref != profile.response_profile_ref {
            return Err(EngineCoreError::InvalidDescriptor(
                "instance profile response profile must match surface family response profile",
            ));
        }
        if !stack
            .layers
            .iter()
            .any(|layer| layer.archetype_id == surface.primary_archetype_ref)
        {
            return Err(EngineCoreError::InvalidDescriptor(
                "instance profile stack must contain the surface primary archetype",
            ));
        }
        if !self
            .response_profiles
            .contains_key(&profile.response_profile_ref)
        {
            return Err(EngineCoreError::InvalidDescriptor(
                "instance profile references unregistered response profile",
            ));
        }
        self.instance_profiles.insert(profile.stack_id, profile);
        Ok(())
    }

    pub fn select_response(
        &self,
        stack_id: crate::material_types::MaterialStackId,
        trigger_class: MaterialTriggerClass,
        current_tier: ConsequenceTier,
        state_modifiers: &[MaterialStateModifier],
    ) -> EngineCoreResult<MaterialConsequenceEvent> {
        let instance =
            self.instance_profiles
                .get(&stack_id)
                .ok_or(EngineCoreError::InvalidDescriptor(
                    "material stack is missing instance profile",
                ))?;
        let profile = self
            .response_profiles
            .get(&instance.response_profile_ref)
            .ok_or(EngineCoreError::InvalidDescriptor(
                "material instance is missing response profile",
            ))?;
        if !profile.supported_trigger_classes.contains(&trigger_class) {
            return Err(EngineCoreError::InvalidDescriptor(
                "material response profile does not support trigger class",
            ));
        }
        let response_family_id = profile.family_for_trigger(trigger_class);
        let response_family = self.response_family_rows.get(response_family_id).ok_or(
            EngineCoreError::InvalidDescriptor("material response family row is not registered"),
        )?;
        if !response_family
            .required_trigger_classes
            .contains(&trigger_class)
        {
            return Err(EngineCoreError::InvalidDescriptor(
                "material response family row rejects trigger class",
            ));
        }

        let minimum_tier = ConsequenceTier::minimum_wake_tier(
            trigger_class,
            profile.wetness_contact_promotes_local,
        );
        let next_tier = current_tier.max(minimum_tier);
        let persistence_required = matches!(
            trigger_class,
            MaterialTriggerClass::RestoreOrStreamIn
                | MaterialTriggerClass::StructuralOverload
                | MaterialTriggerClass::TimeDegradation
        ) || next_tier == ConsequenceTier::FarEcho;

        Ok(MaterialConsequenceEvent::from_response_selection(
            MaterialConsequenceSelection {
                surface_family_id: SurfaceFamilyId::from(instance.surface_family_id.clone()),
                response_family_id: ResponseFamilyId::from(response_family.family_id.clone()),
                response_family_group: response_family.family_group,
                trigger_class,
                previous_tier: current_tier,
                next_tier,
                persistence_required,
                state_modifiers: state_modifiers.to_vec(),
            },
        ))
    }
}

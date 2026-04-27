use crate::material_types::{
    MaterialArchetype, MaterialArchetypeId, MaterialConfig, MaterialDescriptor, MaterialId,
    MaterialLookupResult, MaterialStack, MaterialStackId, ReactionRow, ResponseProfileId,
};
use engine_core::{EngineCoreError, EngineCoreResult};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MaterialRegistry {
    config: MaterialConfig,
    descriptors: BTreeMap<MaterialId, MaterialDescriptor>,
    reactions: BTreeMap<ResponseProfileId, ReactionRow>,
    archetypes: BTreeMap<MaterialArchetypeId, MaterialArchetype>,
    stacks: BTreeMap<MaterialStackId, MaterialStack>,
}

impl MaterialRegistry {
    pub fn new(config: MaterialConfig) -> Self {
        Self {
            config,
            descriptors: BTreeMap::new(),
            reactions: BTreeMap::new(),
            archetypes: BTreeMap::new(),
            stacks: BTreeMap::new(),
        }
    }

    // ========================================================================
    // LEGACY API
    // ========================================================================

    pub fn register_descriptor(&mut self, descriptor: MaterialDescriptor) -> EngineCoreResult<()> {
        if descriptor.property_domains.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "material descriptor requires at least one property domain",
            ));
        }
        self.descriptors.insert(descriptor.material_id, descriptor);
        Ok(())
    }

    pub fn register_reaction(&mut self, reaction: ReactionRow) {
        self.reactions.insert(reaction.response_profile, reaction);
    }

    pub fn descriptor(&self, material_id: MaterialId) -> &MaterialDescriptor {
        self.descriptors
            .get(&material_id)
            .unwrap_or(&self.config.fallback_descriptor)
    }

    pub fn reaction(&self, profile: ResponseProfileId) -> &ReactionRow {
        self.reactions
            .get(&profile)
            .unwrap_or(&self.config.default_reaction)
    }

    pub fn lookup(&self, material_id: MaterialId) -> MaterialLookupResult {
        let used_fallback = !self.descriptors.contains_key(&material_id);
        let descriptor = self.descriptor(material_id).clone();
        let reaction = self.reaction(descriptor.response_profile).clone();
        MaterialLookupResult {
            descriptor,
            reaction,
            used_fallback,
        }
    }

    // ========================================================================
    // PRODUCTION API
    // ========================================================================

    pub fn register_archetype(&mut self, archetype: MaterialArchetype) -> EngineCoreResult<()> {
        if archetype.label.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "material archetype requires non-empty label",
            ));
        }
        if archetype.thickness_mm <= 0.0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "material archetype requires positive thickness",
            ));
        }
        self.archetypes.insert(archetype.id, archetype);
        Ok(())
    }

    pub fn register_stack(&mut self, stack: MaterialStack) -> EngineCoreResult<()> {
        if stack.layers.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "material stack requires at least one layer",
            ));
        }
        for layer in &stack.layers {
            if !self.archetypes.contains_key(&layer.archetype_id) {
                return Err(EngineCoreError::InvalidDescriptor(
                    "material stack references unregistered archetype",
                ));
            }
        }
        self.stacks.insert(stack.id, stack);
        Ok(())
    }

    pub fn archetype(&self, id: MaterialArchetypeId) -> Option<&MaterialArchetype> {
        self.archetypes.get(&id)
    }

    pub fn stack(&self, id: MaterialStackId) -> Option<&MaterialStack> {
        self.stacks.get(&id)
    }
}

use crate::material_response::ConsequenceTier;
use crate::MaterialRegistry;
use engine_core::{EngineCoreError, EngineCoreResult};

impl MaterialRegistry {
    pub fn degrade_consequence_tier(
        &self,
        current_tier: ConsequenceTier,
        target_tier: ConsequenceTier,
        downstream_truth_published: bool,
    ) -> EngineCoreResult<ConsequenceTier> {
        if target_tier > current_tier {
            return Err(EngineCoreError::InvalidDescriptor(
                "consequence tier degradation may not promote tiers",
            ));
        }
        if current_tier == target_tier {
            return Ok(target_tier);
        }
        if current_tier.rank().saturating_sub(target_tier.rank()) > 1 {
            return Err(EngineCoreError::InvalidDescriptor(
                "consequence tier degradation must step down one rung at a time",
            ));
        }
        if current_tier >= ConsequenceTier::DownstreamConsequence && !downstream_truth_published {
            return Err(EngineCoreError::InvalidDescriptor(
                "consequence tier degradation requires published downstream truth",
            ));
        }
        Ok(target_tier)
    }
}

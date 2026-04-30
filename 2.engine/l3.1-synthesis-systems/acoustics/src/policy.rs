use crate::AcousticTier;
use engine_core::StableDigestBuilder;
use engine_material::MaterialLookupResult;

pub(crate) fn stable_acoustic_policy_id(lookup: &MaterialLookupResult) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.acoustics.material_policy")
        .write_u64(u64::from(lookup.descriptor.material_id.0))
        .write_u64(u64::from(lookup.reaction.response_profile.0));
    for coefficient in lookup.reaction.coefficients {
        digest.write_u64(u64::from(coefficient));
    }
    digest.finish().0.max(1)
}

pub(crate) fn select_acoustic_tier(
    lookup: &MaterialLookupResult,
    source_count: usize,
    max_sources: usize,
) -> AcousticTier {
    if source_count == 0 {
        return AcousticTier::Disabled;
    }
    if lookup.used_fallback {
        return AcousticTier::EventOnly;
    }

    let acoustic_load = lookup
        .reaction
        .coefficients
        .iter()
        .map(|coefficient| usize::from(*coefficient))
        .sum::<usize>();

    if source_count >= (max_sources / 2).max(1) || acoustic_load >= 8 {
        AcousticTier::ReducedPropagation
    } else {
        AcousticTier::FullPropagation
    }
}

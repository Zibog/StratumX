use crate::ImagingTier;
use engine_core::StableDigestBuilder;
use engine_material::MaterialLookupResult;

pub(crate) fn stable_material_policy_id(lookup: &MaterialLookupResult) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.imaging.material_policy")
        .write_u64(u64::from(lookup.descriptor.material_id.0))
        .write_u64(u64::from(lookup.reaction.response_profile.0))
        .write_u64(lookup.descriptor.property_domains.len() as u64);
    digest.finish().0.max(1)
}

pub(crate) fn select_imaging_tier(
    lookup: &MaterialLookupResult,
    upload_bytes: usize,
    max_upload_bytes: usize,
) -> ImagingTier {
    if upload_bytes == 0 {
        return ImagingTier::Disabled;
    }
    if lookup.used_fallback {
        return ImagingTier::Impostor;
    }

    let material_weight = lookup.descriptor.property_domains.len().max(1)
        + usize::from(lookup.reaction.response_profile.0);
    let full_threshold = (material_weight * 64).min(max_upload_bytes.max(1));
    let reduced_threshold = (full_threshold / 2).max(1);

    if upload_bytes >= full_threshold {
        ImagingTier::Full
    } else if upload_bytes >= reduced_threshold {
        ImagingTier::Reduced
    } else {
        ImagingTier::Impostor
    }
}

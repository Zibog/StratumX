use crate::ImagingTier;
use engine_core::{EngineCoreResult, StableDigestBuilder};
use engine_material::{MaterialId, MaterialLookupResult};
use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};

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

pub(crate) fn stage_imaging_upload(
    material_id: MaterialId,
    upload_bytes: usize,
) -> EngineCoreResult<Option<engine_transfer_control::TransferResult>> {
    if upload_bytes == 0 {
        return Ok(None);
    }

    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 1,
        max_inflight_uploads: 1,
    });
    transfer
        .submit(TransferRequest {
            asset_key: u64::from(material_id.0),
            compressed_bytes: upload_bytes,
            decoded_bytes: upload_bytes,
            upload_bytes,
        })
        .map(Some)
}

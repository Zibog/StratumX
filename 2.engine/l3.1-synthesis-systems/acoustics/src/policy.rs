use crate::AcousticTier;
use engine_core::{EngineCoreResult, StableDigestBuilder};
use engine_material::{MaterialId, MaterialLookupResult};
use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};

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

pub(crate) fn stage_stream_upload(
    material_id: MaterialId,
    stream_upload_bytes: usize,
) -> EngineCoreResult<Option<engine_transfer_control::TransferResult>> {
    if stream_upload_bytes == 0 {
        return Ok(None);
    }

    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 1,
        max_inflight_uploads: 1,
    });
    transfer
        .submit(TransferRequest {
            asset_key: u64::from(material_id.0),
            compressed_bytes: stream_upload_bytes,
            decoded_bytes: stream_upload_bytes,
            upload_bytes: stream_upload_bytes,
        })
        .map(Some)
}

use crate::policy::{select_acoustic_tier, stable_acoustic_policy_id, stage_stream_upload};
use crate::{AcousticReceipt, AcousticTier, AcousticsConfig, AcousticsRequest, AcousticsResult};
use engine_core::{EngineCoreError, EngineCoreResult, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::ResidencyControlService;
use engine_transfer_control::{TransferControlService, TransferRequest};
use engine_world::WorldState;

#[derive(Debug, Clone)]
pub struct AcousticsService {
    config: AcousticsConfig,
}

impl AcousticsService {
    pub fn new(config: AcousticsConfig) -> Self {
        Self { config }
    }

    pub fn synthesize(
        &self,
        _world: &WorldState,
        _ecs: &EcsSubstrate,
        _materials: &MaterialRegistry,
        _residency: &ResidencyControlService,
        transfer: &mut TransferControlService,
        request: AcousticsRequest,
    ) -> EngineCoreResult<AcousticsResult> {
        if request.source_count > self.config.max_sources {
            return Err(EngineCoreError::InvalidDescriptor(
                "acoustic source count exceeds configured ceiling",
            ));
        }
        if request.stream_upload_bytes > self.config.max_stream_upload_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "acoustic stream upload exceeds configured ceiling",
            ));
        }
        if request.stream_upload_bytes > 0 {
            let _ = transfer.submit(TransferRequest {
                asset_key: 77,
                compressed_bytes: request.stream_upload_bytes.max(1),
                decoded_bytes: request.stream_upload_bytes,
                upload_bytes: request.stream_upload_bytes,
            })?;
        }
        Ok(AcousticsResult {
            synthesized_frames: 1,
            propagated_sources: request.source_count,
        })
    }

    pub fn synthesize_with_receipt(
        &self,
        materials: &MaterialRegistry,
        material_id: MaterialId,
        source_count: usize,
    ) -> EngineCoreResult<AcousticReceipt> {
        if source_count == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "source count must be non-zero",
            ));
        }
        if source_count > self.config.max_sources {
            return Err(EngineCoreError::InvalidDescriptor(
                "source count exceeds ceiling",
            ));
        }

        let material_lookup = materials.lookup(material_id);
        let material_policy_id = stable_acoustic_policy_id(&material_lookup);
        let selected_tier =
            select_acoustic_tier(&material_lookup, source_count, self.config.max_sources);
        let stream_upload_bytes = source_count.saturating_mul(64);
        if stream_upload_bytes > self.config.max_stream_upload_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "stream upload bytes exceed configured acoustic ceiling",
            ));
        }
        let transfer_result = stage_stream_upload(material_id, stream_upload_bytes)?;
        let stream_upload_accepted = transfer_result.is_some();
        let emitter_count = match selected_tier {
            AcousticTier::FullPropagation => source_count,
            AcousticTier::ReducedPropagation => source_count.div_ceil(2),
            AcousticTier::EventOnly => 1,
            AcousticTier::Disabled => 0,
        };

        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.acoustics.receipt")
            .write_u64(u64::from(material_id.0))
            .write_u64(material_policy_id)
            .write_u64(u64::from(material_lookup.descriptor.material_id.0))
            .write_u64(u64::from(material_lookup.reaction.response_profile.0))
            .write_u64(
                material_lookup
                    .reaction
                    .coefficients
                    .iter()
                    .map(|coefficient| u64::from(*coefficient))
                    .sum::<u64>(),
            )
            .write_bool(material_lookup.used_fallback)
            .write_u8(selected_tier as u8)
            .write_u64(source_count as u64)
            .write_u64(emitter_count as u64)
            .write_bool(stream_upload_accepted);

        Ok(AcousticReceipt {
            material_id,
            selected_tier,
            emitter_count,
            used_material_fallback: material_lookup.used_fallback,
            stream_upload_accepted,
            material_policy_id,
            deterministic_digest: digest.finish().0,
        })
    }
}

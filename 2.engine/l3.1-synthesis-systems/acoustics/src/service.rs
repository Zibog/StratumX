use crate::policy::{select_acoustic_tier, stable_acoustic_policy_id};
use crate::{
    AcousticFailure, AcousticFailureReason, AcousticInputs, AcousticReceipt, AcousticResult,
    AcousticTier, AcousticsConfig, AcousticsRequest, AcousticsTransportOutcome,
};
use engine_core::{EngineCoreResult, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::{ResidencyConfig, ResidencyControlService};
use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};
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
        request: AcousticsRequest,
        inputs: AcousticInputs<'_>,
    ) -> AcousticResult<AcousticReceipt> {
        let transport = self.synthesize_transport_only(inputs.transfer, request.clone())?;
        let material_lookup = inputs.materials.lookup(inputs.material_id);
        let material_policy_id = stable_acoustic_policy_id(&material_lookup);
        let selected_tier = select_acoustic_tier(
            &material_lookup,
            request.source_count,
            self.config.max_sources,
        );
        let emitter_count = match selected_tier {
            AcousticTier::FullPropagation => request.source_count,
            AcousticTier::ReducedPropagation => request.source_count.div_ceil(2),
            AcousticTier::EventOnly => usize::from(request.source_count > 0),
            AcousticTier::Disabled => 0,
        };
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.acoustics.receipt")
            .write_u64(u64::from(inputs.material_id.0))
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
            .write_u64(request.source_count as u64)
            .write_u64(emitter_count as u64)
            .write_u64(request.stream_upload_bytes as u64)
            .write_bool(transport.stream_upload_accepted);

        Ok(AcousticReceipt {
            material_id: inputs.material_id,
            selected_tier,
            synthesized_frames: transport.synthesized_frames,
            source_count: request.source_count,
            propagated_sources: transport.propagated_sources,
            stream_upload_bytes: request.stream_upload_bytes,
            emitter_count,
            used_material_fallback: material_lookup.used_fallback,
            stream_upload_accepted: transport.stream_upload_accepted,
            material_policy_id,
            deterministic_digest: digest.finish().0,
        })
    }

    pub fn synthesize_transport_only(
        &self,
        transfer: &mut TransferControlService,
        request: AcousticsRequest,
    ) -> AcousticResult<AcousticsTransportOutcome> {
        if request.source_count == 0 || request.source_count > self.config.max_sources {
            return Err(AcousticFailure::for_reason(
                AcousticFailureReason::InvalidSourceCount,
            ));
        }
        if request.stream_upload_bytes > self.config.max_stream_upload_bytes {
            return Err(AcousticFailure::for_reason(
                AcousticFailureReason::StreamUploadBytesExceeded,
            ));
        }
        let stream_upload_accepted = if request.stream_upload_bytes > 0 {
            transfer
                .submit(TransferRequest {
                    asset_key: 77,
                    compressed_bytes: request.stream_upload_bytes.max(1),
                    decoded_bytes: request.stream_upload_bytes,
                    upload_bytes: request.stream_upload_bytes,
                })
                .map_err(|_| AcousticFailure::for_reason(AcousticFailureReason::BudgetRejected))?;
            true
        } else {
            false
        };

        Ok(AcousticsTransportOutcome {
            synthesized_frames: 1,
            propagated_sources: request.source_count,
            stream_upload_bytes: request.stream_upload_bytes,
            stream_upload_accepted,
        })
    }

    #[deprecated(note = "Use synthesize() receipt-first canonical API")]
    pub fn synthesize_with_receipt(
        &self,
        materials: &MaterialRegistry,
        material_id: MaterialId,
        source_count: usize,
    ) -> AcousticResult<AcousticReceipt> {
        let world = WorldState::new();
        let ecs = EcsSubstrate::new();
        let residency = ResidencyControlService::new(ResidencyConfig {
            resident_item_budget: 1,
            streaming_item_budget: 1,
        });
        let mut transfer = TransferControlService::new(TransferConfig {
            max_inflight_decodes: 1,
            max_inflight_uploads: 1,
        });
        self.synthesize(
            AcousticsRequest {
                source_count,
                stream_upload_bytes: source_count.saturating_mul(64),
            },
            AcousticInputs {
                world: &world,
                ecs: &ecs,
                materials,
                residency: &residency,
                transfer: &mut transfer,
                material_id,
            },
        )
    }

    pub fn synthesize_engine_result(
        &self,
        request: AcousticsRequest,
        inputs: AcousticInputs<'_>,
    ) -> EngineCoreResult<AcousticReceipt> {
        self.synthesize(request, inputs).map_err(Into::into)
    }
}

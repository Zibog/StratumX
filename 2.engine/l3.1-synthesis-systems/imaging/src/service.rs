use crate::policy::{select_imaging_tier, stable_material_policy_id};
use crate::{
    ImagingConfig, ImagingFailure, ImagingFailureReason, ImagingInputs, ImagingReceipt,
    ImagingRequest, ImagingResult, ImagingTier, ImagingTransportOutcome,
};
use engine_core::{EngineCoreResult, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::{ResidencyConfig, ResidencyControlService};
use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};
use engine_world::WorldState;

#[derive(Debug, Clone)]
pub struct ImagingService {
    config: ImagingConfig,
}

impl ImagingService {
    pub fn new(config: ImagingConfig) -> Self {
        Self { config }
    }

    pub fn render(
        &self,
        request: ImagingRequest,
        inputs: ImagingInputs<'_>,
    ) -> ImagingResult<ImagingReceipt> {
        let transport = self.render_transport_only(inputs.transfer, request.clone())?;
        let material_lookup = inputs.materials.lookup(inputs.material_id);
        let material_policy_id = stable_material_policy_id(&material_lookup);
        let selected_tier = select_imaging_tier(
            &material_lookup,
            request.upload_bytes,
            self.config.max_upload_bytes,
        );
        let transient_resources = match selected_tier {
            ImagingTier::Full => 3,
            ImagingTier::Reduced => 2,
            ImagingTier::Impostor => 1,
            ImagingTier::Disabled => 0,
        };
        let resource_count = transient_resources + usize::from(transport.upload_accepted);
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.imaging.receipt")
            .write_u64(request.render_target_id)
            .write_u32(request.view_region.0 as u32)
            .write_u32(request.view_region.1 as u32)
            .write_u32(request.view_region.2 as u32)
            .write_u64(u64::from(inputs.material_id.0))
            .write_u64(material_policy_id)
            .write_u64(u64::from(material_lookup.descriptor.material_id.0))
            .write_u64(u64::from(material_lookup.reaction.response_profile.0))
            .write_u64(material_lookup.descriptor.property_domains.len() as u64)
            .write_bool(material_lookup.used_fallback)
            .write_u8(selected_tier as u8)
            .write_u64(request.upload_bytes as u64)
            .write_bool(transport.upload_accepted)
            .write_u64(resource_count as u64);

        Ok(ImagingReceipt {
            material_id: inputs.material_id,
            render_target_id: request.render_target_id,
            selected_tier,
            rendered_frames: transport.rendered_frames,
            upload_bytes: request.upload_bytes,
            resource_count,
            used_material_fallback: material_lookup.used_fallback,
            upload_accepted: transport.upload_accepted,
            material_policy_id,
            deterministic_digest: digest.finish().0,
        })
    }

    pub fn render_transport_only(
        &self,
        transfer: &mut TransferControlService,
        request: ImagingRequest,
    ) -> ImagingResult<ImagingTransportOutcome> {
        if request.render_target_id as usize > self.config.max_render_targets {
            return Err(ImagingFailure::for_reason(
                ImagingFailureReason::InvalidRenderTarget,
            ));
        }
        if request.upload_bytes > self.config.max_upload_bytes {
            return Err(ImagingFailure::for_reason(
                ImagingFailureReason::UploadBytesExceeded,
            ));
        }
        let upload_accepted = if request.upload_bytes > 0 {
            transfer
                .submit(TransferRequest {
                    asset_key: request.render_target_id,
                    compressed_bytes: request.upload_bytes.max(1),
                    decoded_bytes: request.upload_bytes,
                    upload_bytes: request.upload_bytes,
                })
                .map_err(|_| ImagingFailure::for_reason(ImagingFailureReason::BudgetRejected))?;
            true
        } else {
            false
        };

        Ok(ImagingTransportOutcome {
            rendered_frames: 1,
            upload_bytes: request.upload_bytes,
            upload_accepted,
        })
    }

    #[deprecated(note = "Use render() receipt-first canonical API")]
    pub fn render_with_receipt(
        &self,
        materials: &MaterialRegistry,
        material_id: MaterialId,
        upload_bytes: usize,
    ) -> ImagingResult<ImagingReceipt> {
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
        self.render(
            ImagingRequest {
                render_target_id: u64::from(material_id.0),
                view_region: (0, 0, 0),
                upload_bytes,
            },
            ImagingInputs {
                world: &world,
                ecs: &ecs,
                materials,
                residency: &residency,
                transfer: &mut transfer,
                material_id,
            },
        )
    }

    pub fn render_engine_result(
        &self,
        request: ImagingRequest,
        inputs: ImagingInputs<'_>,
    ) -> EngineCoreResult<ImagingReceipt> {
        self.render(request, inputs).map_err(Into::into)
    }
}

use crate::policy::{select_imaging_tier, stable_material_policy_id, stage_imaging_upload};
use crate::{ImagingConfig, ImagingReceipt, ImagingRequest, ImagingResult, ImagingTier};
use engine_core::{EngineCoreError, EngineCoreResult, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::ResidencyControlService;
use engine_transfer_control::{TransferControlService, TransferRequest};
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
        _world: &WorldState,
        _ecs: &EcsSubstrate,
        _materials: &MaterialRegistry,
        _residency: &ResidencyControlService,
        transfer: &mut TransferControlService,
        request: ImagingRequest,
    ) -> EngineCoreResult<ImagingResult> {
        if request.render_target_id as usize > self.config.max_render_targets {
            return Err(EngineCoreError::InvalidDescriptor(
                "render target id exceeds configured logical bound",
            ));
        }
        if request.upload_bytes > self.config.max_upload_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "upload bytes exceed configured imaging ceiling",
            ));
        }
        if request.upload_bytes > 0 {
            let _ = transfer.submit(TransferRequest {
                asset_key: request.render_target_id,
                compressed_bytes: request.upload_bytes.max(1),
                decoded_bytes: request.upload_bytes,
                upload_bytes: request.upload_bytes,
            })?;
        }
        Ok(ImagingResult {
            rendered_frames: 1,
            upload_bytes: request.upload_bytes,
        })
    }

    pub fn render_with_receipt(
        &self,
        materials: &MaterialRegistry,
        material_id: MaterialId,
        upload_bytes: usize,
    ) -> EngineCoreResult<ImagingReceipt> {
        let material_lookup = materials.lookup(material_id);
        if upload_bytes > self.config.max_upload_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "upload bytes exceed configured imaging ceiling",
            ));
        }

        let material_policy_id = stable_material_policy_id(&material_lookup);
        let selected_tier =
            select_imaging_tier(&material_lookup, upload_bytes, self.config.max_upload_bytes);
        let transfer_result = stage_imaging_upload(material_id, upload_bytes)?;
        let upload_accepted = transfer_result.is_some();
        let transient_resources = match selected_tier {
            ImagingTier::Full => 3,
            ImagingTier::Reduced => 2,
            ImagingTier::Impostor => 1,
            ImagingTier::Disabled => 0,
        };
        let resource_count = transient_resources + usize::from(upload_accepted);

        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.imaging.receipt")
            .write_u64(u64::from(material_id.0))
            .write_u64(material_policy_id)
            .write_u64(u64::from(material_lookup.descriptor.material_id.0))
            .write_u64(u64::from(material_lookup.reaction.response_profile.0))
            .write_u64(material_lookup.descriptor.property_domains.len() as u64)
            .write_bool(material_lookup.used_fallback)
            .write_u8(selected_tier as u8)
            .write_u64(upload_bytes as u64)
            .write_bool(upload_accepted)
            .write_u64(resource_count as u64);

        Ok(ImagingReceipt {
            material_id,
            selected_tier,
            resource_count,
            used_material_fallback: material_lookup.used_fallback,
            upload_accepted,
            material_policy_id,
            deterministic_digest: digest.finish().0,
        })
    }
}

pub mod lighting_runtime;
pub mod texture_residency;

use engine_core::{EngineCoreError, EngineCoreResult};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::ResidencyControlService;
use engine_transfer_control::{TransferControlService, TransferRequest};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub use lighting_runtime::{LightSource, LightType, LightingRuntime, ShadowCaster};
pub use texture_residency::{
    TextureDescriptor, TextureFormat, TextureResidencyInfo, TextureResidencyMetrics,
    TextureResidencyRuntime, TextureResidencyState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingConfig {
    pub max_render_targets: usize,
    pub max_upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingRequest {
    pub render_target_id: u64,
    pub view_region: (i32, i32, i32),
    pub upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagingResult {
    pub rendered_frames: usize,
    pub upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderTarget {
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderView {
    pub region_key: (i32, i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameResource {
    pub target: RenderTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadStage {
    pub bytes: usize,
}

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
        materials: &MaterialRegistry,
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
        let _ = materials.lookup(MaterialId(0));
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
}

pub mod audio_runtime;

use engine_core::{EngineCoreError, EngineCoreResult};
use engine_ecs::EcsSubstrate;
use engine_material::{MaterialId, MaterialRegistry};
use engine_residency_control::ResidencyControlService;
use engine_transfer_control::{TransferControlService, TransferRequest};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub use audio_runtime::{
    AudioRuntime, AudioSource, AudioSourceType, FootstepEvent, FootstepMaterial, Occluder,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsConfig {
    pub max_sources: usize,
    pub max_stream_upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsRequest {
    pub source_count: usize,
    pub stream_upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticsResult {
    pub synthesized_frames: usize,
    pub propagated_sources: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticSource {
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticEnvironment {
    pub region_key: (i32, i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticPropagation {
    pub paths: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcousticOutput {
    pub frames: usize,
}

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
        materials: &MaterialRegistry,
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
        let _ = materials.lookup(MaterialId(0));
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
}

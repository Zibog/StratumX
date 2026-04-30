// Upper Engine Boundary Law Tests
//
// Tests for BLKR-UPPER-ENGINE-01:
// - Generation request digest
// - Inference boundary validation
// - Acoustic event boundary
// - Animation solve boundary
// - Imaging frame policy
// - Content resource policy
// - Startup wiring proof

use engine_acoustics::{AcousticsConfig, AcousticsRequest, AcousticsService};
use engine_animation::{AnimationClip, AnimationRuntime, Keyframe};
use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};
use engine_ecs::EcsSubstrate;
use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
use engine_imaging::{ImagingConfig, ImagingRequest, ImagingService};
use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, ReactionRow,
    ResponseProfileId,
};
use engine_residency_control::{ResidencyConfig, ResidencyControlService};
use engine_runtime::RuntimeProfile;
use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};
use engine_transfer_control::{TransferConfig, TransferControlService};
use engine_world::WorldState;

fn create_test_material_config() -> MaterialConfig {
    MaterialConfig {
        fallback_descriptor: MaterialDescriptor {
            material_id: MaterialId(0),
            label: "fallback".to_string(),
            property_domains: vec![],
            response_profile: ResponseProfileId(0),
        },
        default_reaction: ReactionRow {
            response_profile: ResponseProfileId(0),
            coefficients: [0, 0, 0, 0],
        },
    }
}

fn create_test_service_wiring() -> ServiceWiring {
    ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: false,
        synthesis: false,
    }
}

include!("upper_engine_boundary_law/cases_01.rs");
include!("upper_engine_boundary_law/cases_02.rs");
include!("upper_engine_boundary_law/cases_03.rs");

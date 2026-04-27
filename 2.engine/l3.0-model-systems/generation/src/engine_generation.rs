use engine_core::{EngineCoreError, EngineCoreResult};
use engine_inference::{InferenceRequest, InferenceService};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub max_output_chars: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationResult {
    pub output: GenerationOutput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDescriptor {
    pub model_family: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelWeights {
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationContext {
    pub descriptor: ModelDescriptor,
    pub weights: ModelWeights,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationOutput {
    pub artifact: String,
}

#[derive(Debug, Clone)]
pub struct GenerationService {
    config: GenerationConfig,
    context: GenerationContext,
}

impl GenerationService {
    pub fn new(config: GenerationConfig, context: GenerationContext) -> Self {
        Self { config, context }
    }
    pub fn generate(
        &self,
        _world: &WorldState,
        inference: &InferenceService,
        request: GenerationRequest,
    ) -> EngineCoreResult<GenerationResult> {
        let inference_result = inference.infer_detached(InferenceRequest {
            prompt: request.prompt,
            batch_size: 1,
        })?;
        let mut artifact = format!(
            "{}:{}:",
            self.context.descriptor.model_family, self.context.weights.checksum
        );
        artifact.push_str(&inference_result.tokens.join("-"));
        if artifact.len() > self.config.max_output_chars {
            return Err(EngineCoreError::InvalidDescriptor(
                "generation artifact exceeds configured ceiling",
            ));
        }
        Ok(GenerationResult {
            output: GenerationOutput { artifact },
        })
    }
}

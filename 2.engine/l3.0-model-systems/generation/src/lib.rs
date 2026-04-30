use engine_core::{EngineCoreError, EngineCoreResult, StableDigestBuilder};
use engine_inference::{InferenceRequest, InferenceService};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

/// Failure reasons for model boundary operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelBoundaryFailureReason {
    MissingModelPolicy,
    InvalidInputDescriptor,
    BudgetRejected,
    UnsupportedOutputClass,
}

/// Receipt for model boundary operation with deterministic digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelBoundaryReceipt {
    pub model_policy_id: u64,
    pub input_digest: u64,
    pub output_digest: u64,
    pub deterministic_digest: u64,
}

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
        let output = self.generate_output(inference, request)?;
        if output.artifact.len() > self.config.max_output_chars {
            return Err(EngineCoreError::InvalidDescriptor(
                "generation artifact exceeds configured ceiling",
            ));
        }
        Ok(GenerationResult { output })
    }

    /// Generate with receipt including model policy.
    pub fn generate_with_receipt(
        &self,
        inference: &InferenceService,
        request: GenerationRequest,
    ) -> EngineCoreResult<ModelBoundaryReceipt> {
        let output = self.generate_output(inference, request.clone())?;

        let mut input_digest_builder = StableDigestBuilder::new();
        input_digest_builder
            .write_bytes(b"engine.generation.input")
            .write_bytes(request.prompt.as_bytes());
        let input_digest = input_digest_builder.finish().0;

        let output_digest = self.compute_output_digest(&output);

        let model_policy_id = stable_generation_policy_id(&self.context);
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.generation.receipt")
            .write_u64(model_policy_id)
            .write_bytes(self.context.descriptor.model_family.as_bytes())
            .write_bytes(self.context.weights.checksum.as_bytes())
            .write_u64(input_digest)
            .write_u64(output_digest);

        Ok(ModelBoundaryReceipt {
            model_policy_id,
            input_digest,
            output_digest,
            deterministic_digest: digest.finish().0,
        })
    }

    pub fn compute_output_digest(&self, output: &GenerationOutput) -> u64 {
        let mut output_digest_builder = StableDigestBuilder::new();
        output_digest_builder
            .write_bytes(b"engine.generation.output")
            .write_bytes(output.artifact.as_bytes());
        output_digest_builder.finish().0
    }

    fn generate_output(
        &self,
        inference: &InferenceService,
        request: GenerationRequest,
    ) -> EngineCoreResult<GenerationOutput> {
        validate_context(&self.context)?;
        validate_request(&request)?;

        let inference_result = inference.infer_detached(InferenceRequest {
            prompt: request.prompt,
            batch_size: 1,
        })?;
        let mut artifact = format!(
            "{}:{}:",
            self.context.descriptor.model_family, self.context.weights.checksum
        );
        artifact.push_str(&inference_result.tokens.join("-"));
        Ok(GenerationOutput { artifact })
    }
}

fn stable_generation_policy_id(context: &GenerationContext) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.generation.policy")
        .write_bytes(context.descriptor.model_family.as_bytes())
        .write_bytes(context.weights.checksum.as_bytes());
    digest.finish().0.max(1)
}

fn validate_context(context: &GenerationContext) -> EngineCoreResult<()> {
    if context.descriptor.model_family.trim().is_empty() {
        return Err(EngineCoreError::InvalidDescriptor(
            "generation context requires model family",
        ));
    }
    if context.weights.checksum.trim().is_empty() {
        return Err(EngineCoreError::InvalidDescriptor(
            "generation context requires weights checksum",
        ));
    }
    Ok(())
}

fn validate_request(request: &GenerationRequest) -> EngineCoreResult<()> {
    if request.prompt.trim().is_empty() {
        return Err(EngineCoreError::InvalidDescriptor(
            "generation request requires non-empty prompt",
        ));
    }
    Ok(())
}

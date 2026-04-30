use engine_core::{EngineCoreError, EngineCoreResult, StableDigestBuilder};
use engine_ecs::EcsSubstrate;
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
pub struct InferenceConfig {
    pub max_batch_items: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub prompt: String,
    pub batch_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferenceResult {
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferenceBatch {
    pub requests: Vec<InferenceRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InferenceModel {
    pub model_id: String,
}

#[derive(Debug, Clone)]
pub struct InferenceService {
    config: InferenceConfig,
    model: InferenceModel,
}

impl InferenceService {
    pub fn new(config: InferenceConfig, model: InferenceModel) -> Self {
        Self { config, model }
    }
    pub fn infer(
        &self,
        _world: &WorldState,
        _ecs: &EcsSubstrate,
        request: InferenceRequest,
    ) -> EngineCoreResult<InferenceResult> {
        self.infer_detached(request)
    }
    pub fn infer_detached(&self, request: InferenceRequest) -> EngineCoreResult<InferenceResult> {
        validate_model(&self.model)?;
        validate_request(&request)?;
        if request.batch_size == 0 || request.batch_size > self.config.max_batch_items {
            return Err(EngineCoreError::InvalidDescriptor(
                "inference batch size is illegal for configured ceiling",
            ));
        }
        let mut tokens = vec![self.model.model_id.clone()];
        tokens.extend(request.prompt.split_whitespace().map(ToString::to_string));
        Ok(InferenceResult { tokens })
    }

    /// Infer with receipt including model policy.
    pub fn infer_with_receipt(
        &self,
        request: InferenceRequest,
    ) -> EngineCoreResult<ModelBoundaryReceipt> {
        let result = self.infer_detached(request.clone())?;

        let mut input_digest_builder = StableDigestBuilder::new();
        input_digest_builder
            .write_bytes(b"engine.inference.input")
            .write_bytes(request.prompt.as_bytes())
            .write_u64(request.batch_size as u64);
        let input_digest = input_digest_builder.finish().0;

        let output_digest = self.compute_output_digest(&result);

        let model_policy_id = stable_inference_policy_id(&self.model);
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.inference.receipt")
            .write_u64(model_policy_id)
            .write_bytes(self.model.model_id.as_bytes())
            .write_u64(input_digest)
            .write_u64(output_digest);

        Ok(ModelBoundaryReceipt {
            model_policy_id,
            input_digest,
            output_digest,
            deterministic_digest: digest.finish().0,
        })
    }

    pub fn compute_output_digest(&self, result: &InferenceResult) -> u64 {
        let mut output_digest_builder = StableDigestBuilder::new();
        output_digest_builder.write_bytes(b"engine.inference.output");
        for token in &result.tokens {
            output_digest_builder.write_bytes(token.as_bytes());
        }
        output_digest_builder.finish().0
    }
}

fn stable_inference_policy_id(model: &InferenceModel) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.inference.policy")
        .write_bytes(model.model_id.as_bytes());
    digest.finish().0.max(1)
}

fn validate_model(model: &InferenceModel) -> EngineCoreResult<()> {
    if model.model_id.trim().is_empty() {
        return Err(EngineCoreError::InvalidDescriptor(
            "inference service requires non-empty model id",
        ));
    }
    Ok(())
}

fn validate_request(request: &InferenceRequest) -> EngineCoreResult<()> {
    if request.prompt.trim().is_empty() {
        return Err(EngineCoreError::InvalidDescriptor(
            "inference request requires non-empty prompt",
        ));
    }
    Ok(())
}

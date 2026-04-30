use crate::digest::{input_digest, output_digest, receipt_digest, stable_inference_policy_id};
use crate::{
    InferenceConfig, InferenceModel, InferenceRequest, InferenceResult, ModelBoundaryFailure,
    ModelBoundaryFailureReason, ModelBoundaryReceipt, ModelBoundaryResult,
};
use engine_core::EngineCoreResult;
use engine_ecs::EcsSubstrate;
use engine_world::WorldState;

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
    ) -> ModelBoundaryResult<InferenceResult> {
        self.infer_detached(request)
    }

    pub fn infer_detached(
        &self,
        request: InferenceRequest,
    ) -> ModelBoundaryResult<InferenceResult> {
        validate_model(&self.model)?;
        validate_request(&request)?;
        if request.batch_size == 0 || request.batch_size > self.config.max_batch_items {
            return Err(ModelBoundaryFailure::for_reason(
                ModelBoundaryFailureReason::InvalidBatchSize,
            ));
        }
        let mut tokens = vec![self.model.model_id.clone()];
        tokens.extend(request.prompt.split_whitespace().map(ToString::to_string));
        Ok(InferenceResult { tokens })
    }

    pub fn infer_with_receipt(
        &self,
        request: InferenceRequest,
    ) -> ModelBoundaryResult<ModelBoundaryReceipt> {
        let result = self.infer_detached(request.clone())?;
        let input_digest = input_digest(&request);
        let output_digest = output_digest(&result);
        let model_policy_id = stable_inference_policy_id(&self.model);
        Ok(ModelBoundaryReceipt {
            model_policy_id,
            input_digest,
            output_digest,
            deterministic_digest: receipt_digest(
                &self.model,
                model_policy_id,
                input_digest,
                output_digest,
            ),
        })
    }

    pub fn compute_output_digest(&self, result: &InferenceResult) -> u64 {
        output_digest(result)
    }

    pub fn infer_engine_result(
        &self,
        world: &WorldState,
        ecs: &EcsSubstrate,
        request: InferenceRequest,
    ) -> EngineCoreResult<InferenceResult> {
        self.infer(world, ecs, request).map_err(Into::into)
    }
}

fn validate_model(model: &InferenceModel) -> ModelBoundaryResult<()> {
    if model.model_id.trim().is_empty() {
        return Err(ModelBoundaryFailure::for_reason(
            ModelBoundaryFailureReason::MissingModelId,
        ));
    }
    Ok(())
}

fn validate_request(request: &InferenceRequest) -> ModelBoundaryResult<()> {
    if request.prompt.trim().is_empty() {
        return Err(ModelBoundaryFailure::for_reason(
            ModelBoundaryFailureReason::EmptyPrompt,
        ));
    }
    Ok(())
}

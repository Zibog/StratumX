use crate::digest::{input_digest, output_digest, receipt_digest, stable_generation_policy_id};
use crate::{
    GenerationConfig, GenerationContext, GenerationOutput, GenerationRequest, GenerationResult,
    ModelBoundaryFailure, ModelBoundaryFailureReason, ModelBoundaryReceipt, ModelBoundaryResult,
};
use engine_core::EngineCoreResult;
use engine_inference::{
    InferenceRequest, InferenceService, ModelBoundaryFailureReason as InferenceFailureReason,
};
use engine_world::WorldState;

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
    ) -> ModelBoundaryResult<GenerationResult> {
        let output = self.generate_output(inference, request)?;
        if output.artifact.len() > self.config.max_output_chars {
            return Err(ModelBoundaryFailure::for_reason(
                ModelBoundaryFailureReason::OutputTooLarge,
            ));
        }
        Ok(GenerationResult { output })
    }

    pub fn generate_with_receipt(
        &self,
        inference: &InferenceService,
        request: GenerationRequest,
    ) -> ModelBoundaryResult<ModelBoundaryReceipt> {
        let output = self.generate_output(inference, request.clone())?;
        let input_digest = input_digest(&request);
        let output_digest = output_digest(&output);
        let model_policy_id = stable_generation_policy_id(&self.context);
        Ok(ModelBoundaryReceipt {
            model_policy_id,
            input_digest,
            output_digest,
            deterministic_digest: receipt_digest(
                &self.context,
                model_policy_id,
                input_digest,
                output_digest,
            ),
        })
    }

    pub fn compute_output_digest(&self, output: &GenerationOutput) -> u64 {
        output_digest(output)
    }

    pub fn generate_engine_result(
        &self,
        world: &WorldState,
        inference: &InferenceService,
        request: GenerationRequest,
    ) -> EngineCoreResult<GenerationResult> {
        self.generate(world, inference, request).map_err(Into::into)
    }

    fn generate_output(
        &self,
        inference: &InferenceService,
        request: GenerationRequest,
    ) -> ModelBoundaryResult<GenerationOutput> {
        validate_context(&self.context)?;
        validate_request(&request)?;
        let inference_result = inference
            .infer_detached(InferenceRequest {
                prompt: request.prompt,
                batch_size: 1,
            })
            .map_err(map_inference_failure)?;
        let mut artifact = format!(
            "{}:{}:",
            self.context.descriptor.model_family, self.context.weights.checksum
        );
        artifact.push_str(&inference_result.tokens.join("-"));
        Ok(GenerationOutput { artifact })
    }
}

fn validate_context(context: &GenerationContext) -> ModelBoundaryResult<()> {
    if context.descriptor.model_family.trim().is_empty() {
        return Err(ModelBoundaryFailure::for_reason(
            ModelBoundaryFailureReason::MissingModelFamily,
        ));
    }
    if context.weights.checksum.trim().is_empty() {
        return Err(ModelBoundaryFailure::for_reason(
            ModelBoundaryFailureReason::MissingWeightsChecksum,
        ));
    }
    Ok(())
}

fn validate_request(request: &GenerationRequest) -> ModelBoundaryResult<()> {
    if request.prompt.trim().is_empty() {
        return Err(ModelBoundaryFailure::for_reason(
            ModelBoundaryFailureReason::EmptyPrompt,
        ));
    }
    Ok(())
}

fn map_inference_failure(failure: engine_inference::ModelBoundaryFailure) -> ModelBoundaryFailure {
    match failure.reason {
        InferenceFailureReason::EmptyPrompt => {
            ModelBoundaryFailure::for_reason(ModelBoundaryFailureReason::EmptyPrompt)
        }
        _ => ModelBoundaryFailure::for_reason(ModelBoundaryFailureReason::InferenceUnavailable),
    }
}

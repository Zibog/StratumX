use crate::{InferenceModel, InferenceRequest, InferenceResult};
use engine_core::StableDigestBuilder;

pub fn stable_inference_policy_id(model: &InferenceModel) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.inference.policy")
        .write_bytes(model.model_id.as_bytes());
    digest.finish().0.max(1)
}

pub fn input_digest(request: &InferenceRequest) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.inference.input")
        .write_bytes(request.prompt.as_bytes())
        .write_u64(request.batch_size as u64);
    digest.finish().0
}

pub fn output_digest(result: &InferenceResult) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest.write_bytes(b"engine.inference.output");
    for token in &result.tokens {
        digest.write_bytes(token.as_bytes());
    }
    digest.finish().0
}

pub fn receipt_digest(
    model: &InferenceModel,
    model_policy_id: u64,
    input_digest: u64,
    output_digest: u64,
) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.inference.receipt")
        .write_u64(model_policy_id)
        .write_bytes(model.model_id.as_bytes())
        .write_u64(input_digest)
        .write_u64(output_digest);
    digest.finish().0
}

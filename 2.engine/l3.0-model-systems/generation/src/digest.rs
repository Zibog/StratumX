use crate::{GenerationContext, GenerationOutput, GenerationRequest};
use engine_core::StableDigestBuilder;

pub fn stable_generation_policy_id(context: &GenerationContext) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.generation.policy")
        .write_bytes(context.descriptor.model_family.as_bytes())
        .write_bytes(context.weights.checksum.as_bytes());
    digest.finish().0.max(1)
}

pub fn input_digest(request: &GenerationRequest) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.generation.input")
        .write_bytes(request.prompt.as_bytes());
    digest.finish().0
}

pub fn output_digest(output: &GenerationOutput) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.generation.output")
        .write_bytes(output.artifact.as_bytes());
    digest.finish().0
}

pub fn receipt_digest(
    context: &GenerationContext,
    model_policy_id: u64,
    input_digest: u64,
    output_digest: u64,
) -> u64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.generation.receipt")
        .write_u64(model_policy_id)
        .write_bytes(context.descriptor.model_family.as_bytes())
        .write_bytes(context.weights.checksum.as_bytes())
        .write_u64(input_digest)
        .write_u64(output_digest);
    digest.finish().0
}

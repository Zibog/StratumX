#![allow(unused_imports)]
use super::*;
use engine_inference::{InferenceConfig, InferenceModel, InferenceService};

fn inf() -> InferenceService {
    InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    )
}
#[test]
fn generation_composes_model_family_and_checksum() {
    let g = GenerationService::new(
        GenerationConfig {
            max_output_chars: 128,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    assert!(g
        .generate(
            &engine_world::WorldState::new(),
            &inf(),
            GenerationRequest {
                prompt: "hello".to_string()
            }
        )
        .unwrap()
        .output
        .artifact
        .contains("gen:abc"));
}
#[test]
fn generation_rejects_artifacts_over_limit() {
    let g = GenerationService::new(
        GenerationConfig {
            max_output_chars: 4,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    assert!(g
        .generate(
            &engine_world::WorldState::new(),
            &inf(),
            GenerationRequest {
                prompt: "hello".to_string()
            }
        )
        .is_err());
}
#[test]
fn generation_calls_inference() {
    let g = GenerationService::new(
        GenerationConfig {
            max_output_chars: 128,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    let out = g
        .generate(
            &engine_world::WorldState::new(),
            &inf(),
            GenerationRequest {
                prompt: "hello world".to_string(),
            },
        )
        .unwrap();
    assert!(out.output.artifact.contains("mock"));
}
#[test]
fn generation_accepts_short_prompt() {
    let g = GenerationService::new(
        GenerationConfig {
            max_output_chars: 128,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    assert!(g
        .generate(
            &engine_world::WorldState::new(),
            &inf(),
            GenerationRequest {
                prompt: "hi".to_string()
            }
        )
        .is_ok());
}
#[test]
fn generation_output_is_non_empty() {
    let g = GenerationService::new(
        GenerationConfig {
            max_output_chars: 128,
        },
        GenerationContext {
            descriptor: ModelDescriptor {
                model_family: "gen".to_string(),
            },
            weights: ModelWeights {
                checksum: "abc".to_string(),
            },
        },
    );
    assert!(!g
        .generate(
            &engine_world::WorldState::new(),
            &inf(),
            GenerationRequest {
                prompt: "hello".to_string()
            }
        )
        .unwrap()
        .output
        .artifact
        .is_empty());
}

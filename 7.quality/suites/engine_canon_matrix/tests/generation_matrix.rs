#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn generation_generate_case_strategy() -> impl Strategy<Value = usize> {
    0usize..30
}

proptest! {
    #[test]
    fn generation_all_cases(|case in generation_generate_case_strategy()) {
        let max_output_chars = 128 + (case as u32) * 8;
        let inf = InferenceService::new(
            InferenceConfig { max_batch_items: 8 },
            InferenceModel {
                model_id: "mock".to_string(),
            },
        );
        let gen = GenerationService::new(
            GenerationConfig { max_output_chars: max_output_chars as usize },
            GenerationContext {
                descriptor: ModelDescriptor {
                    model_family: "gen".to_string(),
                },
                weights: ModelWeights {
                    checksum: "abc".to_string(),
                },
            },
        );
        let r = gen
            .generate(
                &WorldState::new(),
                &inf,
                GenerationRequest {
                    prompt: "hello".to_string(),
                },
            )
            .unwrap();
        prop_assert!(r.output.artifact.contains("gen:abc"));
    }
}

use engine_core::{EngineCoreError, EngineCoreResult};
use engine_ecs::EcsSubstrate;
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

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
        if request.batch_size == 0 || request.batch_size > self.config.max_batch_items {
            return Err(EngineCoreError::InvalidDescriptor(
                "inference batch size is illegal for configured ceiling",
            ));
        }
        let mut tokens = vec![self.model.model_id.clone()];
        tokens.extend(request.prompt.split_whitespace().map(ToString::to_string));
        Ok(InferenceResult { tokens })
    }
}

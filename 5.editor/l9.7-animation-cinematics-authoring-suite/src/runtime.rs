//! Runtime integration for editor-to-engine animation connections

use crate::model::AnimationModel;

/// Bridge between editor animation models and engine runtime
pub struct AnimationRuntimeBridge {
    pending_sync: Vec<AnimationModel>,
}

impl AnimationRuntimeBridge {
    pub fn new() -> Self {
        Self {
            pending_sync: Vec::new(),
        }
    }

    /// Queue an animation model for sync to the engine
    pub fn queue_for_sync(&mut self, model: AnimationModel) {
        self.pending_sync.push(model);
    }

    /// Flush all pending animation models to engine
    pub fn flush_pending(&mut self) -> Vec<AnimationModel> {
        std::mem::take(&mut self.pending_sync)
    }

    /// Convert an editor animation model to engine-compatible format
    pub fn model_to_engine_format(&self, model: &AnimationModel) -> EngineAnimationClip {
        EngineAnimationClip {
            name: model.name.clone(),
            duration: model.duration,
            loop_enabled: model.loop_enabled,
        }
    }
}

/// Engine-compatible animation clip representation
#[derive(Debug, Clone)]
pub struct EngineAnimationClip {
    pub name: String,
    pub duration: f32,
    pub loop_enabled: bool,
}

impl Default for AnimationRuntimeBridge {
    fn default() -> Self {
        Self::new()
    }
}

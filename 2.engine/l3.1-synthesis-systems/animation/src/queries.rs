use crate::animation_runtime::{AnimationRuntime, AnimationState};

pub fn get_animation_states(runtime: &AnimationRuntime) -> &[AnimationState] {
    &runtime.states
}

pub fn list_animation_clip_names(runtime: &AnimationRuntime) -> Vec<&str> {
    runtime.clips.iter().map(|c| c.name.as_str()).collect()
}

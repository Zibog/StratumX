use crate::animation_runtime::AnimationClip;

pub fn validate_animation_clip(clip: &AnimationClip) -> Result<(), String> {
    if clip.keyframes.is_empty() {
        return Err("Animation clip must have at least one keyframe".to_string());
    }
    if clip.duration_sec <= 0.0 {
        return Err("Animation clip duration must be positive".to_string());
    }
    if clip.name.is_empty() {
        return Err("Animation clip name must not be empty".to_string());
    }
    Ok(())
}

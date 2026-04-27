//! Validation logic

use crate::timeline::Timeline;
use crate::animation_clip::AnimationClip;

pub fn validate_timeline(timeline: &Timeline) -> Result<(), ValidationError> {
    if timeline.name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    if timeline.duration <= 0.0 {
        return Err(ValidationError::InvalidDuration);
    }
    if timeline.framerate <= 0.0 {
        return Err(ValidationError::InvalidFramerate);
    }
    Ok(())
}

pub fn validate_clip(clip: &AnimationClip) -> Result<(), ValidationError> {
    if clip.name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    if clip.duration <= 0.0 {
        return Err(ValidationError::InvalidDuration);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    EmptyName,
    InvalidDuration,
    InvalidFramerate,
}

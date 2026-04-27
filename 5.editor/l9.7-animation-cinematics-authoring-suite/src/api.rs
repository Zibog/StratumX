//! Public API for animation and cinematics authoring

use crate::timeline::*;
use crate::animation_clip::*;
use crate::cinematic_sequence::*;
use std::collections::HashMap;

/// Animation authoring service
pub struct AnimationAuthoringService {
    timelines: HashMap<TimelineId, Timeline>,
    clips: HashMap<AnimationClipId, AnimationClip>,
    sequences: HashMap<CinematicSequenceId, CinematicSequence>,
}

impl AnimationAuthoringService {
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
            clips: HashMap::new(),
            sequences: HashMap::new(),
        }
    }

    pub fn create_timeline(&mut self, name: String, duration: f32) -> TimelineId {
        let timeline = Timeline::new(name, duration);
        let id = timeline.id;
        self.timelines.insert(id, timeline);
        id
    }

    pub fn get_timeline(&self, id: TimelineId) -> Option<&Timeline> {
        self.timelines.get(&id)
    }

    pub fn get_timeline_mut(&mut self, id: TimelineId) -> Option<&mut Timeline> {
        self.timelines.get_mut(&id)
    }

    pub fn create_clip(&mut self, name: String, duration: f32) -> AnimationClipId {
        let clip = AnimationClip::new(name, duration);
        let id = clip.id;
        self.clips.insert(id, clip);
        id
    }

    pub fn get_clip(&self, id: AnimationClipId) -> Option<&AnimationClip> {
        self.clips.get(&id)
    }

    pub fn get_clip_mut(&mut self, id: AnimationClipId) -> Option<&mut AnimationClip> {
        self.clips.get_mut(&id)
    }

    pub fn create_sequence(&mut self, name: String) -> CinematicSequenceId {
        let sequence = CinematicSequence::new(name);
        let id = sequence.id;
        self.sequences.insert(id, sequence);
        id
    }

    pub fn get_sequence(&self, id: CinematicSequenceId) -> Option<&CinematicSequence> {
        self.sequences.get(&id)
    }

    pub fn get_sequence_mut(&mut self, id: CinematicSequenceId) -> Option<&mut CinematicSequence> {
        self.sequences.get_mut(&id)
    }
}

impl Default for AnimationAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}

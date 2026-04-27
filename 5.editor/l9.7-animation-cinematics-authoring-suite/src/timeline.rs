//! Timeline management for animations and cinematics

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Timeline ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimelineId(pub Uuid);

impl TimelineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Timeline track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub name: String,
    pub track_type: TrackType,
    pub keyframes: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Transform,
    Animation,
    Audio,
    Camera,
    Event,
}

/// Timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub id: TimelineId,
    pub name: String,
    pub duration: f32,
    pub framerate: f32,
    pub tracks: Vec<Track>,
    pub loop_enabled: bool,
}

impl Timeline {
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            id: TimelineId::new(),
            name,
            duration,
            framerate: 30.0,
            tracks: Vec::new(),
            loop_enabled: false,
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, track_id: Uuid) {
        self.tracks.retain(|t| t.id != track_id);
    }

    pub fn get_track(&self, track_id: Uuid) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == track_id)
    }

    pub fn get_track_mut(&mut self, track_id: Uuid) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == track_id)
    }
}

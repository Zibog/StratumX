//! Cinematic sequence composition

use crate::timeline::TimelineId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Cinematic sequence ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CinematicSequenceId(pub Uuid);

impl CinematicSequenceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Cinematic shot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicShot {
    pub id: Uuid,
    pub name: String,
    pub start_time: f32,
    pub duration: f32,
    pub camera_timeline: Option<TimelineId>,
}

/// Cinematic sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicSequence {
    pub id: CinematicSequenceId,
    pub name: String,
    pub shots: Vec<CinematicShot>,
    pub total_duration: f32,
}

impl CinematicSequence {
    pub fn new(name: String) -> Self {
        Self {
            id: CinematicSequenceId::new(),
            name,
            shots: Vec::new(),
            total_duration: 0.0,
        }
    }

    pub fn add_shot(&mut self, shot: CinematicShot) {
        let end_time = shot.start_time + shot.duration;
        if end_time > self.total_duration {
            self.total_duration = end_time;
        }
        self.shots.push(shot);
        self.shots.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
    }

    pub fn remove_shot(&mut self, shot_id: Uuid) {
        self.shots.retain(|s| s.id != shot_id);
        self.recalculate_duration();
    }

    fn recalculate_duration(&mut self) {
        self.total_duration = self.shots
            .iter()
            .map(|s| s.start_time + s.duration)
            .fold(0.0, f32::max);
    }
}

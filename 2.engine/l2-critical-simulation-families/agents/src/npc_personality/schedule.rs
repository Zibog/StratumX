use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    Idle,
    Working,
    Eating,
    Sleeping,
    Socializing,
    Traveling,
    Criminal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleState {
    pub current_activity: ActivityType,
    pub activity_start_time: f32,
    pub activity_duration: f32,
    pub location: [f32; 3],
}

impl ScheduleState {
    pub fn new() -> Self {
        Self {
            current_activity: ActivityType::Idle,
            activity_start_time: 0.0,
            activity_duration: 0.0,
            location: [0.0, 0.0, 0.0],
        }
    }

    pub fn start_activity(
        &mut self,
        activity: ActivityType,
        start_time: f32,
        duration: f32,
        location: [f32; 3],
    ) {
        self.current_activity = activity;
        self.activity_start_time = start_time;
        self.activity_duration = duration;
        self.location = location;
    }

    pub fn is_activity_finished(&self, current_time: f32) -> bool {
        current_time >= self.activity_start_time + self.activity_duration
    }
}

impl Default for ScheduleState {
    fn default() -> Self {
        Self::new()
    }
}

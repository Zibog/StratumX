use super::door_types::{DoorObject, DoorState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathStatus {
    Valid,
    Blocked,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationPath {
    pub start: [f32; 3],
    pub destination: [f32; 3],
    pub status: PathStatus,
    pub blocked_reason: Option<String>,
}

impl NavigationPath {
    pub fn new(start: [f32; 3], destination: [f32; 3]) -> Self {
        Self {
            start,
            destination,
            status: PathStatus::Valid,
            blocked_reason: None,
        }
    }

    pub fn update_from_door(&mut self, door: &DoorObject) {
        let door_pos = door.position;
        let is_door_on_path = self.is_point_on_path(door_pos);

        if is_door_on_path {
            match door.state {
                DoorState::Open => {
                    self.status = PathStatus::Valid;
                    self.blocked_reason = None;
                }
                DoorState::Closed => {
                    self.status = PathStatus::Blocked;
                    self.blocked_reason = Some("Door is closed".to_string());
                }
                DoorState::Locked => {
                    self.status = PathStatus::Blocked;
                    self.blocked_reason = Some("Door is locked".to_string());
                }
                DoorState::Blocked => {
                    self.status = PathStatus::Blocked;
                    self.blocked_reason = door.blocked_reason.clone();
                }
            }
        }
    }

    fn is_point_on_path(&self, point: [f32; 3]) -> bool {
        let dx = self.destination[0] - self.start[0];
        let dz = self.destination[2] - self.start[2];
        let px = point[0] - self.start[0];
        let pz = point[2] - self.start[2];

        let dot = px * dx + pz * dz;
        let len_sq = dx * dx + dz * dz;

        if len_sq == 0.0 {
            return false;
        }

        let t = (dot / len_sq).clamp(0.0, 1.0);
        let closest_x = self.start[0] + t * dx;
        let closest_z = self.start[2] + t * dz;

        let dist_sq = (point[0] - closest_x).powi(2) + (point[2] - closest_z).powi(2);
        dist_sq < 4.0
    }
}

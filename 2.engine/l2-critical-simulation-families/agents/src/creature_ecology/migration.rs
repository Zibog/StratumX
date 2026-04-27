use serde::{Deserialize, Serialize};

use super::creature_profile::MigrationReason;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationCorridor {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub width: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationState {
    pub current_position: [f32; 3],
    pub destination: Option<[f32; 3]>,
    pub corridor: Option<MigrationCorridor>,
    pub active: bool,
    pub reason: Option<MigrationReason>,
}

impl MigrationState {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            current_position: position,
            destination: None,
            corridor: None,
            active: false,
            reason: None,
        }
    }

    pub fn start_migration(&mut self, destination: [f32; 3], reason: MigrationReason) {
        self.destination = Some(destination);
        self.active = true;
        self.reason = Some(reason);

        self.corridor = Some(MigrationCorridor {
            start: self.current_position,
            end: destination,
            width: 50.0,
        });
    }

    pub fn update_position(&mut self, new_position: [f32; 3]) {
        self.current_position = new_position;

        if let Some(dest) = self.destination {
            let distance =
                ((dest[0] - new_position[0]).powi(2) + (dest[2] - new_position[2]).powi(2)).sqrt();

            if distance < 5.0 {
                self.active = false;
                self.destination = None;
                self.corridor = None;
                self.reason = None;
            }
        }
    }
}

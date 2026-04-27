use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoorState {
    Open,
    Closed,
    Locked,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorObject {
    pub position: [f32; 3],
    pub state: DoorState,
    pub interaction_point: [f32; 3],
    pub blocked_reason: Option<String>,
}

impl DoorObject {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
            state: DoorState::Closed,
            interaction_point: [position[0], position[1] + 1.0, position[2]],
            blocked_reason: None,
        }
    }

    pub fn open(&mut self) -> Result<(), String> {
        match self.state {
            DoorState::Closed => {
                self.state = DoorState::Open;
                Ok(())
            }
            DoorState::Locked => Err("Door is locked".to_string()),
            DoorState::Blocked => Err(format!(
                "Door is blocked: {}",
                self.blocked_reason
                    .as_ref()
                    .unwrap_or(&"unknown reason".to_string())
            )),
            DoorState::Open => Ok(()),
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        match self.state {
            DoorState::Open => {
                self.state = DoorState::Closed;
                Ok(())
            }
            _ => Err("Door cannot be closed in current state".to_string()),
        }
    }

    pub fn set_blocked(&mut self, reason: String) {
        self.state = DoorState::Blocked;
        self.blocked_reason = Some(reason);
    }

    pub fn set_locked(&mut self) {
        self.state = DoorState::Locked;
        self.blocked_reason = Some("Locked".to_string());
    }
}

// World state access methods

use super::WorldLifecycleManager;
use editor_dto_law::{StableWorldId, WorldBindState};
use engine_world::WorldState;

impl WorldLifecycleManager {
    pub fn get_current_world(&self) -> Option<StableWorldId> {
        self.current_world
    }

    pub fn get_bind_state(&self) -> Option<&WorldBindState> {
        self.bind_state.as_ref()
    }

    pub fn update_bind_state(&mut self, state: WorldBindState) {
        self.bind_state = Some(state);
    }

    pub fn get_world_state(&self) -> Option<&WorldState> {
        self.world_state.as_ref()
    }

    pub fn get_world_state_mut(&mut self) -> Option<&mut WorldState> {
        self.world_state.as_mut()
    }

    pub fn take_world_state(&mut self) -> Option<WorldState> {
        self.world_state.take()
    }
}

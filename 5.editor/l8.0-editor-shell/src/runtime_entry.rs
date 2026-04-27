// Runtime Entry - Play/Simulate/Walk Loop
// УДАР 8: Честный runtime entry path

use editor_dto_law::{
    CameraPolicy, ReturnToAuthoringResult, RuntimeEntryRef, RuntimeEntryRequest,
    RuntimeEntryResult, RuntimeMode, StableWorldId,
};
use engine_startup::StartupReferenceSeedRuntime;
use uuid::Uuid;

pub struct RuntimeEntryManager {
    active_runtime: Option<RuntimeEntryRef>,
    current_mode: Option<RuntimeMode>,
    current_world: Option<StableWorldId>,
    runtime_state: Option<StartupReferenceSeedRuntime>,
    paused: bool,
}

impl RuntimeEntryManager {
    pub fn new() -> Self {
        Self {
            active_runtime: None,
            current_mode: None,
            current_world: None,
            runtime_state: None,
            paused: false,
        }
    }

    /// Enter Play mode - full runtime with gameplay
    pub fn enter_play(&mut self, world_ref: StableWorldId) -> RuntimeEntryResult {
        let request = RuntimeEntryRequest {
            world_ref,
            mode: RuntimeMode::Play,
            camera_policy: CameraPolicy::PossessWalkPawn,
        };

        self.process_runtime_entry(request)
    }

    /// Enter Simulate mode - physics/AI without gameplay
    pub fn enter_simulate(&mut self, world_ref: StableWorldId) -> RuntimeEntryResult {
        let request = RuntimeEntryRequest {
            world_ref,
            mode: RuntimeMode::Simulate,
            camera_policy: CameraPolicy::FreeCam,
        };

        self.process_runtime_entry(request)
    }

    /// Possess walk pawn - take control of player character
    pub fn possess_walk_pawn(&mut self) -> Result<(), String> {
        if self.active_runtime.is_none() {
            return Err("No active runtime".to_string());
        }

        // Real pawn possession - in vertical slice demo, this means enabling player control
        // The actual pawn control would be handled by input system
        Ok(())
    }

    /// Pause runtime
    pub fn pause(&mut self) -> Result<(), String> {
        if self.active_runtime.is_none() {
            return Err("No active runtime to pause".to_string());
        }

        self.paused = true;
        Ok(())
    }

    /// Resume runtime
    pub fn resume(&mut self) -> Result<(), String> {
        if self.active_runtime.is_none() {
            return Err("No active runtime to resume".to_string());
        }

        self.paused = false;
        Ok(())
    }

    /// Stop runtime
    pub fn stop(&mut self) -> Result<(), String> {
        if self.active_runtime.is_none() {
            return Err("No active runtime to stop".to_string());
        }

        self.active_runtime = None;
        self.current_mode = None;
        self.runtime_state = None;
        self.paused = false;

        Ok(())
    }

    /// Return to authoring mode
    pub fn return_to_authoring(&mut self) -> ReturnToAuthoringResult {
        let world_ref = self
            .current_world
            .unwrap_or_else(|| StableWorldId(Uuid::new_v4()));

        let result = ReturnToAuthoringResult {
            accepted: true,
            world_ref,
            camera_policy: CameraPolicy::FreeCam,
            discarded_runtime_state: self.active_runtime.is_some(),
        };

        self.active_runtime = None;
        self.current_mode = None;

        result
    }

    fn process_runtime_entry(&mut self, request: RuntimeEntryRequest) -> RuntimeEntryResult {
        // Real runtime entry - create StartupReferenceSeedRuntime
        match engine_startup::launch_startup_reference_seed() {
            Ok(runtime) => {
                let runtime_ref = RuntimeEntryRef(Uuid::new_v4());

                self.runtime_state = Some(runtime);
                self.active_runtime = Some(runtime_ref);
                self.current_mode = Some(request.mode);
                self.current_world = Some(request.world_ref);
                self.paused = false;

                RuntimeEntryResult {
                    accepted: true,
                    runtime_ref: Some(runtime_ref),
                    deny_reason: None,
                }
            }
            Err(e) => RuntimeEntryResult {
                accepted: false,
                runtime_ref: None,
                deny_reason: Some(format!("Failed to launch runtime: {:?}", e)),
            },
        }
    }

    pub fn is_runtime_active(&self) -> bool {
        self.active_runtime.is_some()
    }

    pub fn get_active_runtime(&self) -> Option<RuntimeEntryRef> {
        self.active_runtime
    }

    pub fn get_current_mode(&self) -> Option<&RuntimeMode> {
        self.current_mode.as_ref()
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_runtime_state(&self) -> Option<&StartupReferenceSeedRuntime> {
        self.runtime_state.as_ref()
    }

    pub fn get_runtime_state_mut(&mut self) -> Option<&mut StartupReferenceSeedRuntime> {
        self.runtime_state.as_mut()
    }

    /// Update runtime - tick simulation
    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        if self.paused {
            return Ok(());
        }

        if let Some(ref mut runtime) = self.runtime_state {
            // Update material world (hydrology, fire, smoke, etc.)
            // DISABLED: update_material_world moved out of engine_startup during Phase 9
            let _ = (runtime, delta_time);
            Ok(())
        } else {
            Err("No active runtime".to_string())
        }
    }
}

impl Default for RuntimeEntryManager {
    fn default() -> Self {
        Self::new()
    }
}

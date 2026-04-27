// Vertical Slice Runtime - Startup and Session
// Contains VerticalSliceSession and core bootstrap/reset logic

use engine_startup::{launch_startup_reference_seed, StartupReferenceSeedRuntime};
use link_egress_observations::*;
use link_ingress_packets::{
    VerticalSliceCommand, VerticalSliceIngressPacket, VerticalSliceSessionHandle,
};

pub struct VerticalSliceSession {
    pub runtime: StartupReferenceSeedRuntime,
}

impl VerticalSliceSessionHandle for VerticalSliceSession {
    fn handle_command(
        &mut self,
        packet: VerticalSliceIngressPacket,
    ) -> Result<VerticalSliceObservation, String> {
        match packet.command {
            VerticalSliceCommand::BootstrapScene => self.bootstrap_scene(packet.request_id),
            VerticalSliceCommand::FireTestShot { weapon_entity_id } => {
                self.fire_test_shot(packet.request_id, weapon_entity_id)
            }
            VerticalSliceCommand::ResetScene => self.reset_scene(packet.request_id),
            VerticalSliceCommand::SelectEntity { .. } => {
                Err("SelectEntity not implemented in vertical slice".into())
            }
            VerticalSliceCommand::AssignMaterialStack { .. } => {
                Err("AssignMaterialStack not implemented in vertical slice".into())
            }
        }
    }
}

impl VerticalSliceSession {
    pub fn new() -> Result<Self, String> {
        let runtime = launch_startup_reference_seed().map_err(|e| format!("{:?}", e))?;
        Ok(Self { runtime })
    }

    pub fn bootstrap_scene(&mut self, request_id: u64) -> Result<VerticalSliceObservation, String> {
        let scene_dto = self.extract_scene_dto()?;
        // extract_material_stacks and extract_damage_memory disabled - references
        // fields moved out of engine_startup during Phase 9 refactoring.
        let damage_memory = self.extract_damage_memory()?;

        Ok(VerticalSliceObservation {
            request_id,
            scene: Some(scene_dto),
            material_stacks: vec![],
            damage_memory,
            ballistic_result: None,
            runtime_events: vec![],
            shot_log: vec![],
        })
    }

    pub fn reset_scene(&mut self, request_id: u64) -> Result<VerticalSliceObservation, String> {
        self.runtime = launch_startup_reference_seed().map_err(|e| format!("{:?}", e))?;
        self.bootstrap_scene(request_id)
    }

    // DISABLED: fire_test_shot and helpers
    // References ballistics, materials, damage_applicator fields moved out of engine_startup
    // during Phase 9 refactoring. Restored when kinetics/material domains are re-integrated.
    pub fn fire_test_shot(
        &mut self,
        _request_id: u64,
        _weapon_entity_id: u32,
    ) -> Result<VerticalSliceObservation, String> {
        Err(
            "fire_test_shot temporarily disabled - kinetics domain moved out of engine_startup"
                .into(),
        )
    }

    // Disabled helpers referenced by fire_test_shot:
    // - convert_impact_result
    // - convert_verdict
}

impl Default for VerticalSliceSession {
    fn default() -> Self {
        Self::new().expect("Failed to create vertical slice session")
    }
}

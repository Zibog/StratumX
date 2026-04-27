// Authoring Session State

use crate::VerticalSliceSession;
use link_egress_observations::*;
use link_ingress_packets::*;
use std::collections::HashMap;

pub struct EditorAuthoringSession {
    pub(super) vertical_slice_session: Option<VerticalSliceSession>,

    pub(super) next_asset_id: u32,
    pub(super) next_archetype_id: u16,
    pub(super) next_stack_id: u16,
    pub(super) next_actor_id: u32,

    pub(super) active_scene: Option<String>,
    pub(super) assets: HashMap<u32, AssetDto>,
    pub(super) archetypes: HashMap<u16, MaterialArchetypeDto>,
    pub(super) stacks: HashMap<u16, AuthoringMaterialStackDto>,
    pub(super) actors: HashMap<u32, ActorDto>,
    pub(super) active_actor: Option<u32>,

    // Material world state
    pub(super) fire_object_burning: bool,
    pub(super) barrel_water_liters: f32,
}

impl Default for EditorAuthoringSession {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorAuthoringSession {
    pub fn new() -> Self {
        Self {
            vertical_slice_session: None,
            next_asset_id: 1,
            next_archetype_id: 1,
            next_stack_id: 1,
            next_actor_id: 1,
            active_scene: None,
            assets: HashMap::new(),
            archetypes: HashMap::new(),
            stacks: HashMap::new(),
            actors: HashMap::new(),
            active_actor: None,
            fire_object_burning: false,
            barrel_water_liters: 0.0,
        }
    }

    pub fn initialize_vertical_slice_session(&mut self) -> Result<(), String> {
        let session = VerticalSliceSession::new()?;
        let scene_name = session.authoring_get_scene_summary()?.scene_name;
        self.vertical_slice_session = Some(session);
        // Only set active_scene if not already set (e.g., by CreateEmpty with a custom name)
        if self.active_scene.is_none() {
            self.active_scene = Some(scene_name);
        }
        Ok(())
    }

    pub fn has_runtime_session(&self) -> bool {
        self.vertical_slice_session.is_some()
    }

    pub fn get_world_summary(&self) -> WorldSummaryDto {
        if let Some(vs) = &self.vertical_slice_session {
            if let Ok(scene) = vs.authoring_get_scene_summary() {
                return WorldSummaryDto {
                    active_scene: self
                        .active_scene
                        .clone()
                        .or_else(|| Some(scene.scene_name.clone())),
                    entity_count: 3,
                    active_actor: self.active_actor,
                };
            }
        }

        WorldSummaryDto {
            active_scene: self.active_scene.clone(),
            entity_count: 0,
            active_actor: self.active_actor,
        }
    }

    pub fn handle_command(
        &mut self,
        packet: EditorAuthoringPacket,
    ) -> Result<EditorAuthoringObservation, String> {
        match packet.command {
            EditorAuthoringCommand::Scene(cmd) => super::scene_commands::handle(self, cmd),
            EditorAuthoringCommand::Material(cmd) => super::material_commands::handle(self, cmd),
            EditorAuthoringCommand::Terrain(cmd) => super::terrain_commands::handle(self, cmd),
            EditorAuthoringCommand::Actor(cmd) => super::actor_commands::handle(self, cmd),
            EditorAuthoringCommand::Asset(cmd) => super::asset_commands::handle(self, cmd),
            EditorAuthoringCommand::Ballistics(cmd) => {
                super::ballistics_commands::handle(self, cmd)
            }
            EditorAuthoringCommand::InspectReasonChain(cmd) => {
                super::reason_chain_commands::handle(self, cmd)
            }
            EditorAuthoringCommand::MaterialWorld(cmd) => super::world_commands::handle(self, cmd),
            EditorAuthoringCommand::Destruction(cmd) => {
                super::destruction_commands::handle(self, cmd)
            }
            EditorAuthoringCommand::NavDoorInventory(cmd) => {
                super::nav_door_commands::handle(self, cmd)
            }
            EditorAuthoringCommand::Population(cmd) => {
                super::population_commands::handle(self, cmd)
            }
            EditorAuthoringCommand::Tactics(cmd) => super::tactics_commands::handle(self, cmd),
            EditorAuthoringCommand::Ecology(cmd) => super::ecology_commands::handle(self, cmd),
            EditorAuthoringCommand::Sky(cmd) => super::sky_commands::handle(self, cmd),
            EditorAuthoringCommand::Storm(cmd) => super::storm_commands::handle(self, cmd),
            EditorAuthoringCommand::Animation(cmd) => {
                super::animation_commands::handle(self, cmd)
            }
        }
    }
}

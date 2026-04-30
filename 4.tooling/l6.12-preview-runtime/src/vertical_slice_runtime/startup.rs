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
        let material_stacks = self.extract_material_stacks()?;
        let damage_memory = self.extract_damage_memory()?;

        Ok(VerticalSliceObservation {
            request_id,
            scene: Some(scene_dto),
            material_stacks,
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

    // fire_test_shot - ballistic simulation with damage application
    pub fn fire_test_shot(
        &mut self,
        request_id: u64,
        weapon_entity_id: u32,
    ) -> Result<VerticalSliceObservation, String> {
        use engine_kinetics::ProjectileProfileId;

        // Get weapon from scene
        let scene = self
            .runtime
            .world
            .vertical_slice_scene()
            .ok_or("Scene not found")?;

        if scene.weapon.entity_id.0 != weapon_entity_id {
            return Err(format!(
                "Weapon entity {} not found in scene",
                weapon_entity_id
            ));
        }

        // Spawn projectile
        let projectile = self
            .runtime
            .ballistic_simulator
            .spawn_projectile(
                ProjectileProfileId(1),
                scene.weapon.position,
                scene.weapon.aim_direction,
            )
            .ok_or("Failed to spawn projectile")?;

        // Simulate ballistic trajectory
        let mut projectile_state = projectile;
        let dt_s = 0.001; // 1ms timestep
        let max_steps = 1000;

        for _ in 0..max_steps {
            self.runtime
                .ballistic_simulator
                .integrate_step(&mut projectile_state, dt_s)
                .map_err(|e| format!("{:?}", e))?;

            // Check wall hit
            if let Some((impact_pos, angle_deg)) = self.runtime.ballistic_simulator.check_wall_hit(
                &projectile_state,
                scene.wall.position,
                scene.wall.dimensions,
            ) {
                // Get material stack from world
                let materials = self.extract_material_registry()?;
                let stack_id = engine_material::MaterialStackId(scene.wall.stack_id.0);

                // Resolve impact
                let impact_result = self
                    .runtime
                    .ballistic_simulator
                    .resolve_impact(
                        &projectile_state,
                        scene.wall.entity_id.0,
                        impact_pos,
                        angle_deg,
                        stack_id,
                        &materials,
                    )
                    .map_err(|e| format!("{:?}", e))?;

                // Apply damage to world
                self.apply_damage_from_impact(&impact_result)?;

                // Extract observation
                let damage_memory = self.extract_damage_memory()?;
                let runtime_events = self.extract_runtime_events();
                let shot_log = self.extract_shot_log();

                return Ok(VerticalSliceObservation {
                    request_id,
                    scene: None,
                    material_stacks: vec![],
                    damage_memory,
                    ballistic_result: Some(self.convert_ballistic_result(impact_result)),
                    runtime_events,
                    shot_log,
                });
            }
        }

        Err("Projectile did not hit target within simulation time".into())
    }

    fn extract_material_registry(&self) -> Result<engine_material::MaterialRegistry, String> {
        use engine_material::{
            concrete, MaterialConfig, MaterialDescriptor, MaterialId, MaterialLayer, MaterialStack,
            MaterialStackId, PropertyDomain, ReactionRow, ResponseProfileId,
        };

        // Create registry with proper config
        let config = MaterialConfig {
            fallback_descriptor: MaterialDescriptor {
                material_id: MaterialId(0),
                label: "fallback".into(),
                property_domains: vec![PropertyDomain::Physical],
                response_profile: ResponseProfileId(0),
            },
            default_reaction: ReactionRow {
                response_profile: ResponseProfileId(0),
                coefficients: [1, 1, 1, 1],
            },
        };
        let mut registry = engine_material::MaterialRegistry::new(config);

        // Get the wall material stack from scene
        let scene = self
            .runtime
            .world
            .vertical_slice_scene()
            .ok_or("Scene not found")?;

        let stack_id = MaterialStackId(scene.wall.stack_id.0);

        // Register concrete archetype
        let archetype = concrete();
        registry
            .register_archetype(archetype)
            .map_err(|e| format!("{:?}", e))?;

        // Create stack with concrete layer
        let stack = MaterialStack {
            id: stack_id,
            label: "wall_concrete".to_string(),
            layers: vec![MaterialLayer {
                archetype_id: engine_material::MaterialArchetypeId(3), // concrete ID
                thickness_mm: 200.0,
                coverage: 1.0,
            }],
        };

        registry
            .register_stack(stack)
            .map_err(|e| format!("{:?}", e))?;

        Ok(registry)
    }

    fn apply_damage_from_impact(
        &mut self,
        impact: &engine_kinetics::ImpactResult,
    ) -> Result<(), String> {
        use engine_world::EntityId;

        let entity_id = EntityId(impact.target_entity);
        let scene = self
            .runtime
            .world
            .vertical_slice_scene()
            .ok_or("Scene not found")?;
        let stack_id = scene.wall.stack_id;

        // Calculate damage for each layer
        for layer_event in &impact.layer_events {
            let integrity_loss = layer_event.energy_absorbed_j / 10000.0; // Simple damage model

            // Update damage memory in world
            let damage_memory = self.runtime.world.damage_memory_mut();
            for mem in damage_memory.iter_mut() {
                if mem.entity_id == entity_id && mem.stack_id == stack_id {
                    if let Some(layer) = mem.layer_damage.get_mut(layer_event.layer_index as usize)
                    {
                        layer.integrity = (layer.integrity - integrity_loss).max(0.0);
                        layer.accumulated_energy_j += layer_event.energy_absorbed_j;
                    }
                }
            }
        }

        Ok(())
    }

    fn convert_ballistic_result(
        &self,
        impact: engine_kinetics::ImpactResult,
    ) -> link_egress_observations::BallisticSimulationResultDto {
        link_egress_observations::BallisticSimulationResultDto {
            impacts: vec![link_egress_observations::ImpactResultDto {
                target_entity: impact.target_entity,
                impact_position: impact.impact_position,
                impact_velocity: impact.impact_velocity,
                entry_energy_j: impact.entry_energy_j,
                incidence_angle_deg: impact.incidence_angle_deg,
                layer_events: impact
                    .layer_events
                    .iter()
                    .map(|e| link_egress_observations::LayerImpactEventDto {
                        layer_index: e.layer_index,
                        entry_energy_j: e.entry_energy_j,
                        exit_energy_j: e.exit_energy_j,
                        energy_absorbed_j: e.energy_absorbed_j,
                        verdict: self.convert_verdict(e.verdict),
                    })
                    .collect(),
                final_verdict: self.convert_verdict(impact.final_verdict),
            }],
        }
    }

    fn convert_verdict(
        &self,
        verdict: engine_kinetics::ImpactVerdict,
    ) -> link_egress_observations::ImpactVerdictDto {
        match verdict {
            engine_kinetics::ImpactVerdict::Stopped => {
                link_egress_observations::ImpactVerdictDto::Stopped
            }
            engine_kinetics::ImpactVerdict::Penetrated => {
                link_egress_observations::ImpactVerdictDto::Penetrated
            }
            engine_kinetics::ImpactVerdict::Ricochet => {
                link_egress_observations::ImpactVerdictDto::Ricochet
            }
            engine_kinetics::ImpactVerdict::Embedded => {
                link_egress_observations::ImpactVerdictDto::Embedded
            }
        }
    }
}

impl Default for VerticalSliceSession {
    fn default() -> Self {
        Self::new().expect("Failed to create vertical slice session")
    }
}

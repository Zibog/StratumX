// Vertical Slice Runtime - Extract functions
// Scene, damage, events extraction

use engine_world::RuntimeEvent;
use link_egress_observations::*;

impl super::startup::VerticalSliceSession {
    pub fn extract_scene_dto(&self) -> Result<SceneDto, String> {
        let scene = self
            .runtime
            .world
            .vertical_slice_scene()
            .ok_or("Scene not found")?;

        Ok(SceneDto {
            scene_name: scene.scene_name.clone(),
            terrain: TerrainPatchDto {
                entity_id: scene.terrain.entity_id.0,
                origin: scene.terrain.origin,
                world_size: scene.terrain.world_size,
                resolution: scene.terrain.resolution,
                mesh_revision: scene.terrain.mesh_revision,
            },
            wall: WallDto {
                entity_id: scene.wall.entity_id.0,
                position: scene.wall.position,
                dimensions: scene.wall.dimensions,
                stack_id: scene.wall.stack_id.0,
            },
            weapon: WeaponDto {
                entity_id: scene.weapon.entity_id.0,
                profile_id: scene.weapon.profile_id.0,
                position: scene.weapon.position,
                aim_direction: scene.weapon.aim_direction,
            },
            camera: CameraDto {
                position: scene.camera.position,
                look_at: scene.camera.look_at,
                fov_deg: scene.camera.fov_deg,
            },
            sky_bundle_path: scene.sky_bundle_path.clone(),
        })
    }

    // extract_material_stacks DISABLED - references runtime.materials field
    // moved out of engine_startup during Phase 9 refactoring.

    pub fn extract_material_stacks(&self) -> Result<Vec<MaterialStackDto>, String> {
        use engine_material::concrete;

        let scene = self
            .runtime
            .world
            .vertical_slice_scene()
            .ok_or("Scene not found")?;

        let concrete_archetype = concrete();

        Ok(vec![MaterialStackDto {
            id: scene.wall.stack_id.0,
            label: "wall_concrete".to_string(),
            layers: vec![MaterialLayerDto {
                archetype_id: concrete_archetype.id.0,
                archetype_label: concrete_archetype.label.clone(),
                thickness_mm: concrete_archetype.thickness_mm,
                coverage: 1.0,
            }],
        }])
    }

    pub fn extract_damage_memory(&self) -> Result<Vec<DamageMemoryDto>, String> {
        let damage_memory = self.runtime.world.damage_memory();
        Ok(damage_memory
            .iter()
            .map(|mem| DamageMemoryDto {
                entity_id: mem.entity_id.0,
                stack_id: mem.stack_id.0,
                layer_damage: mem
                    .layer_damage
                    .iter()
                    .map(|layer| LayerDamageDto {
                        layer_index: layer.layer_index,
                        integrity: layer.integrity,
                        accumulated_energy_j: layer.accumulated_energy_j,
                        cracked_segments: layer.cracked_segments.clone(),
                        released_segments: layer.released_segments.clone(),
                    })
                    .collect(),
            })
            .collect())
    }

    pub fn extract_runtime_events(&self) -> Vec<RuntimeEventDto> {
        self.runtime
            .world
            .runtime_events()
            .iter()
            .map(|event| match event {
                RuntimeEvent::ShotFired { tick, weapon } => RuntimeEventDto::ShotFired {
                    tick: tick.0,
                    weapon: weapon.0,
                },
                RuntimeEvent::ProjectileImpact {
                    tick,
                    target,
                    energy_j,
                } => RuntimeEventDto::ProjectileImpact {
                    tick: tick.0,
                    target: target.0,
                    energy_j: *energy_j,
                },
                RuntimeEvent::SegmentCracked {
                    tick,
                    entity,
                    layer,
                    segment,
                } => RuntimeEventDto::SegmentCracked {
                    tick: tick.0,
                    entity: entity.0,
                    layer: *layer,
                    segment: *segment,
                },
                RuntimeEvent::SegmentReleased {
                    tick,
                    entity,
                    layer,
                    segment,
                } => RuntimeEventDto::SegmentReleased {
                    tick: tick.0,
                    entity: entity.0,
                    layer: *layer,
                    segment: *segment,
                },
            })
            .collect()
    }

    pub fn extract_shot_log(&self) -> Vec<ShotLogEntryDto> {
        self.runtime
            .world
            .shot_log()
            .iter()
            .map(|shot| ShotLogEntryDto {
                tick: shot.tick.0,
                weapon_entity: shot.weapon_entity.0,
                projectile_profile: shot.projectile_profile.0,
                spawn_position: shot.spawn_position,
                spawn_velocity: shot.spawn_velocity,
            })
            .collect()
    }
}

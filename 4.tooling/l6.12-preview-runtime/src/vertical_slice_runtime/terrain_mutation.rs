// Vertical Slice Runtime - Scene mutation stubs
// DISABLED: Most methods reference functions moved out of engine_startup during Phase 9 refactoring.
// Only position/material update helpers and scene summary extraction remain.

use link_egress_observations::SceneDto;

impl super::startup::VerticalSliceSession {
    // SCENE POSITION UPDATES (these work by directly mutating the scene)

    pub fn authoring_update_terrain_position(
        &mut self,
        new_position: [f32; 3],
    ) -> Result<(), String> {
        let scene = self
            .runtime
            .world
            .vertical_slice_scene_mut()
            .ok_or("Scene not found")?;
        scene.terrain.origin = new_position;
        Ok(())
    }

    pub fn authoring_update_wall_position(&mut self, new_position: [f32; 3]) -> Result<(), String> {
        let scene = self
            .runtime
            .world
            .vertical_slice_scene_mut()
            .ok_or("Scene not found")?;
        scene.wall.position = new_position;
        Ok(())
    }

    pub fn authoring_update_wall_material(&mut self, stack_id: u16) -> Result<(), String> {
        let scene = self
            .runtime
            .world
            .vertical_slice_scene_mut()
            .ok_or("Scene not found")?;
        scene.wall.stack_id = engine_world::MaterialStackId(stack_id);
        Ok(())
    }

    pub fn authoring_update_weapon_position(
        &mut self,
        new_position: [f32; 3],
    ) -> Result<(), String> {
        let scene = self
            .runtime
            .world
            .vertical_slice_scene_mut()
            .ok_or("Scene not found")?;
        scene.weapon.position = new_position;
        Ok(())
    }

    pub fn authoring_update_weapon_aim(&mut self, new_direction: [f32; 3]) -> Result<(), String> {
        let scene = self
            .runtime
            .world
            .vertical_slice_scene_mut()
            .ok_or("Scene not found")?;
        scene.weapon.aim_direction = new_direction;
        Ok(())
    }

    // Scene summary (delegates to extract_scene_dto)
    pub fn authoring_get_scene_summary(&self) -> Result<SceneDto, String> {
        self.extract_scene_dto()
    }

    // DISABLED - reference methods moved out of engine_startup during Phase 9:
    // - Terrain material, blast response
    // - Barrel water, leak
    // - Fire object (ignite, extinguish, wetness, state, smoke)
    // - Material world update
    // - Wall integrity, destroyed state
    // - Support object (type, state, damage)
    // - Destruction summary, reset
}

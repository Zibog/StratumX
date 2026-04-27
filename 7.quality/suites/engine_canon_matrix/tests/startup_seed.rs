//! Startup Seed Tests

#[cfg(test)]
mod tests {
    use stratumx_test_support::create_world_with_scene;

    #[test]
    fn test_startup_from_seed() {
        let world = create_world_with_scene();
        let scene = world
            .vertical_slice_scene()
            .expect("startup fixture should include a scene");

        assert_eq!(scene.scene_name, "Test Scene");
        assert_eq!(scene.terrain.height_samples.len(), 1);
        assert_eq!(scene.camera.fov_deg, 60.0);
    }
}

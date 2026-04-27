//! End-to-End Tests: World Open/Save/Import/Sky Loop
//!
//! PHASE 08: REAL INTEGRATION TESTS
//!
//! These tests verify the complete product loop:
//! 1. Create project -> Create world -> Save world -> Close world -> Reopen world
//! 2. Terrain import -> Terrain rebuild -> Viewport update
//! 3. Environment changes (sky, weather) -> Save -> Reopen -> Verify

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    use stratumx_editor_app::editor_host::EditorHost;

    /// Test: Create a new project, create a world, save it, close it, reopen it
    #[test]
    fn test_project_create_world_open_save_reopen_cycle() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path().to_path_buf();
        let project_name = "test_project";
        let world_name = "test_world";

        // Step 1: Create project (simulates EditorHost.create_project_with_world)
        let mut host = EditorHost::new();
        let world_dir = host
            .create_project_with_world(project_name, project_root.to_str().unwrap(), world_name)
            .expect("Failed to create project");

        assert!(world_dir.exists(), "World directory should exist");
        assert!(
            world_dir.join("world.json").exists(),
            "world.json should exist after project creation"
        );

        // Step 2: Open the world
        let open_result = host.open_world_from_path(&world_dir);
        assert!(
            open_result.is_ok(),
            "World should open successfully: {:?}",
            open_result
        );

        // Step 3: Modify the world (add some terrain/environment changes)
        {
            let session = host.session.as_ref().expect("Session should exist");
            let scene = session
                .world
                .vertical_slice_scene()
                .expect("Vertical slice scene should exist");

            // Verify initial state
            assert_eq!(scene.sky.celestial.time_of_day_hours, 14.0);
        }

        // Step 4: Save the world
        let save_result = host.save_world_to_path(&world_dir);
        assert!(
            save_result.is_ok(),
            "World should save successfully: {:?}",
            save_result
        );

        // Step 5: Verify world.json was written
        let world_json_content = fs::read_to_string(world_dir.join("world.json"))
            .expect("world.json should be readable");
        assert!(
            world_json_content.contains(world_name),
            "world.json should contain world name"
        );

        // Step 6: Close the world
        host.session = None;
        assert!(host.session.is_none(), "World should be closed");

        // Step 7: Reopen the world
        let mut host2 = EditorHost::new();
        let reopen_result = host2.open_world_from_path(&world_dir);
        assert!(
            reopen_result.is_ok(),
            "World should reopen successfully: {:?}",
            reopen_result
        );

        // Step 8: Verify world state is restored
        {
            let session = host2
                .session
                .as_ref()
                .expect("Session should exist after reopen");
            let scene = session
                .world
                .vertical_slice_scene()
                .expect("Vertical slice scene should exist");

            // Verify environment was restored
            assert_eq!(scene.sky.celestial.time_of_day_hours, 14.0);
            assert_eq!(scene.terrain.resolution, [256, 256]);
        }
    }

    /// Test: Terrain import updates world state and can be saved/reloaded
    #[test]
    fn test_terrain_import_save_reload_cycle() {
        let temp_dir = TempDir::new().unwrap();
        let world_dir = temp_dir.path().join("terrain_test");
        fs::create_dir_all(&world_dir).unwrap();

        // Create initial world.json
        let initial_world = serde_json::json!({
            "world_name": "terrain_world",
            "version": "0.1.0",
            "terrain": {
                "resolution": [256, 256],
                "world_size": [1000.0, 1000.0],
                "chunk_grid": [4, 4]
            },
            "environment": {
                "weather_regime": "Clear",
                "time_of_day_hours": 12.0,
                "cloud_coverage": 0.0
            }
        });
        fs::write(
            world_dir.join("world.json"),
            serde_json::to_string_pretty(&initial_world).unwrap(),
        )
        .unwrap();

        // Create a simple 256x256 raw heightmap
        let heightmap_data: Vec<u8> = (0..(256 * 256)).map(|i| (i % 256) as u8).collect();
        let heightmap_path = world_dir.join("heightmap.raw");
        fs::write(&heightmap_path, &heightmap_data).unwrap();

        // Open world and import terrain
        let mut host = EditorHost::new();
        let open_result = host.open_world_from_path(&world_dir);
        assert!(open_result.is_ok(), "World should open: {:?}", open_result);

        let import_result = host.import_heightmap_into_active_world(&heightmap_path);
        assert!(
            import_result.is_ok(),
            "Terrain import should succeed: {:?}",
            import_result
        );

        // Verify terrain was updated
        {
            let session = host.session.as_ref().expect("Session should exist");
            let scene = session
                .world
                .vertical_slice_scene()
                .expect("Scene should exist");

            assert_eq!(scene.terrain.resolution, [256, 256]);
            assert_eq!(scene.terrain.height_samples.len(), 256 * 256);
            assert!(
                !scene.terrain.height_samples.is_empty(),
                "Height samples should exist"
            );
        }

        // Save the world
        let save_result = host.save_world_to_path(&world_dir);
        assert!(
            save_result.is_ok(),
            "Save should succeed: {:?}",
            save_result
        );

        // Close and reopen
        host.session = None;
        let mut host2 = EditorHost::new();
        let reopen_result = host2.open_world_from_path(&world_dir);
        assert!(
            reopen_result.is_ok(),
            "Reopen should succeed: {:?}",
            reopen_result
        );

        // Verify terrain persisted
        {
            let session = host2.session.as_ref().expect("Session should exist");
            let scene = session
                .world
                .vertical_slice_scene()
                .expect("Scene should exist");

            assert_eq!(scene.terrain.resolution, [256, 256]);
            assert_eq!(scene.terrain.height_samples.len(), 256 * 256);
        }
    }

    /// Test: Environment changes (sky, weather) persist through save/reload
    #[test]
    fn test_environment_changes_save_reload_cycle() {
        let temp_dir = TempDir::new().unwrap();
        let world_dir = temp_dir.path().join("environment_test");
        fs::create_dir_all(&world_dir).unwrap();

        // Create initial world
        let mut host = EditorHost::new();
        let create_result = host.create_project_with_world(
            "env_project",
            world_dir.parent().unwrap().to_str().unwrap(),
            "env_world",
        );
        assert!(create_result.is_ok(), "Project creation should succeed");

        // Use the created world path
        let actual_world_dir = temp_dir
            .path()
            .join("env_project")
            .join("worlds")
            .join("env_world");

        let open_result = host.open_world_from_path(&actual_world_dir);
        assert!(open_result.is_ok(), "World should open: {:?}", open_result);

        // Modify environment
        {
            let scene = host
                .session
                .as_mut()
                .expect("Session should exist")
                .world
                .vertical_slice_scene_mut()
                .expect("Scene should exist");

            // Change time of day
            scene.sky.celestial.time_of_day_hours = 18.5;
            // Change weather
            scene.sky.weather_director.target_regime = engine_material::WeatherRegime::Overcast;
            // Change cloud coverage
            scene.sky.cloud_coverage = 0.8;
            // Change fog density
            scene.sky.set_fog_density(0.5);
        }

        // Save the world
        let save_result = host.save_world_to_path(&actual_world_dir);
        assert!(
            save_result.is_ok(),
            "Save should succeed: {:?}",
            save_result
        );

        // Verify world.json contains updated values
        let world_json_content = fs::read_to_string(actual_world_dir.join("world.json"))
            .expect("world.json should be readable");
        assert!(
            world_json_content.contains("18.5"),
            "Should contain new time"
        );
        assert!(
            world_json_content.contains("Overcast"),
            "Should contain new weather"
        );
        assert!(
            world_json_content.contains("0.8"),
            "Should contain new cloud coverage"
        );

        // Close and reopen
        host.session = None;
        let mut host2 = EditorHost::new();
        let reopen_result = host2.open_world_from_path(&actual_world_dir);
        assert!(
            reopen_result.is_ok(),
            "Reopen should succeed: {:?}",
            reopen_result
        );

        // Verify environment was restored
        {
            let scene = host2
                .session
                .as_ref()
                .expect("Session should exist")
                .world
                .vertical_slice_scene()
                .expect("Scene should exist");

            assert_eq!(scene.sky.celestial.time_of_day_hours, 18.5);
            assert_eq!(scene.sky.cloud_coverage, 0.8);
        }
    }

    /// Test: World identity is preserved across save/load cycles
    #[test]
    fn test_world_identity_preservation() {
        let temp_dir = TempDir::new().unwrap();
        let world_dir = temp_dir.path().join("identity_test");
        fs::create_dir_all(&world_dir).unwrap();

        // Create and open world
        let mut host = EditorHost::new();
        host.create_project_with_world("id_project", temp_dir.path().to_str().unwrap(), "id_world")
            .unwrap();

        let actual_world_dir = temp_dir
            .path()
            .join("id_project")
            .join("worlds")
            .join("id_world");
        host.open_world_from_path(&actual_world_dir).unwrap();

        // Get initial world state
        let initial_session = host.session.as_ref().expect("Session should exist");
        let initial_world_ref = initial_session.world_ref;

        // Save, close, reopen
        host.save_world_to_path(&actual_world_dir).unwrap();
        host.session = None;

        let mut host2 = EditorHost::new();
        host2.open_world_from_path(&actual_world_dir).unwrap();

        // Verify identity preserved
        let final_session = host2.session.as_ref().expect("Session should exist");
        let final_world_ref = final_session.world_ref;

        assert_eq!(
            initial_world_ref, final_world_ref,
            "World reference should be preserved across save/load"
        );
    }

    /// Test: Multiple save cycles don't corrupt world state
    #[test]
    fn test_multiple_save_cycles() {
        let temp_dir = TempDir::new().unwrap();
        let world_dir = temp_dir.path().join("multi_save_test");
        fs::create_dir_all(&world_dir).unwrap();

        let mut host = EditorHost::new();
        host.create_project_with_world(
            "multi_project",
            temp_dir.path().to_str().unwrap(),
            "multi_world",
        )
        .unwrap();

        let actual_world_dir = temp_dir
            .path()
            .join("multi_project")
            .join("worlds")
            .join("multi_world");
        host.open_world_from_path(&actual_world_dir).unwrap();

        // Save 5 times
        for i in 0..5 {
            // Make a small change
            {
                let scene = host
                    .session
                    .as_mut()
                    .unwrap()
                    .world
                    .vertical_slice_scene_mut()
                    .unwrap();
                scene.sky.celestial.time_of_day_hours = 10.0 + (i as f32);
            }

            let save_result = host.save_world_to_path(&actual_world_dir);
            assert!(save_result.is_ok(), "Save {} should succeed", i);

            // Verify world.json is valid after each save
            let content = fs::read_to_string(actual_world_dir.join("world.json"))
                .expect("world.json should be readable");
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&content);
            assert!(
                parsed.is_ok(),
                "world.json should be valid JSON after save {}",
                i
            );
        }

        // Final reopen and verification
        host.session = None;
        let mut host2 = EditorHost::new();
        let reopen_result = host2.open_world_from_path(&actual_world_dir);
        assert!(
            reopen_result.is_ok(),
            "Final reopen should succeed: {:?}",
            reopen_result
        );

        let final_scene = host2
            .session
            .as_ref()
            .unwrap()
            .world
            .vertical_slice_scene()
            .unwrap();

        // Should have the last value (14.0)
        assert_eq!(final_scene.sky.celestial.time_of_day_hours, 14.0);
    }
}

#[cfg(test)]
mod quality_gates {
    use link_ingress_packets::{VerticalSliceIngressPacket, VerticalSliceSessionHandle};
    use stratumx_tooling_l6_12_preview_runtime::VerticalSliceSession;

    #[test]
    fn smoke_app_boots_vertical_slice_demo() {
        let session = VerticalSliceSession::new();
        assert!(session.is_ok(), "Session should boot successfully");
    }

    #[test]
    fn smoke_fire_shot_roundtrip_works() {
        let mut session = VerticalSliceSession::new().expect("create session");
        let packet = VerticalSliceIngressPacket::fire_test_shot(1, 3);
        let result = session.handle_command(packet);
        assert!(result.is_ok(), "Fire shot should succeed");

        let observation = result.unwrap();
        assert!(
            observation.ballistic_result.is_some(),
            "Should have ballistic result"
        );
        assert!(
            !observation.damage_memory.is_empty(),
            "Should have damage memory"
        );
    }

    #[test]
    fn integration_dto_extract_inspector_pipeline_valid() {
        let mut session = VerticalSliceSession::new().expect("create session");
        let bootstrap_packet = VerticalSliceIngressPacket::bootstrap_scene(1);
        let observation = session.handle_command(bootstrap_packet).expect("bootstrap");

        assert!(observation.scene.is_some(), "Scene DTO should exist");
        assert!(
            !observation.material_stacks.is_empty(),
            "Material stacks should exist"
        );
        assert!(
            !observation.damage_memory.is_empty(),
            "Damage memory should exist"
        );

        let scene = observation.scene.unwrap();
        assert_eq!(scene.scene_name, "vertical_slice.wall_terrain.ak_demo");
        assert_eq!(scene.wall.entity_id, 2);
    }

    #[test]
    fn determinism_same_seed_same_result() {
        let mut session1 = VerticalSliceSession::new().expect("create session 1");
        let mut session2 = VerticalSliceSession::new().expect("create session 2");

        let packet1 = VerticalSliceIngressPacket::fire_test_shot(1, 3);
        let packet2 = VerticalSliceIngressPacket::fire_test_shot(1, 3);

        let obs1 = session1.handle_command(packet1).expect("fire shot 1");
        let obs2 = session2.handle_command(packet2).expect("fire shot 2");

        assert_eq!(
            obs1.ballistic_result.is_some(),
            obs2.ballistic_result.is_some()
        );

        if let (Some(r1), Some(r2)) = (&obs1.ballistic_result, &obs2.ballistic_result) {
            assert_eq!(r1.impacts.len(), r2.impacts.len());
            if !r1.impacts.is_empty() && !r2.impacts.is_empty() {
                assert_eq!(r1.impacts[0].final_verdict, r2.impacts[0].final_verdict);
            }
        }
    }

    #[test]
    fn boundedness_debris_event_counts_respect_limits() {
        let mut session = VerticalSliceSession::new().expect("create session");
        let packet = VerticalSliceIngressPacket::fire_test_shot(1, 3);
        let observation = session.handle_command(packet).expect("fire shot");

        const MAX_EVENTS: usize = 1000;
        assert!(
            observation.runtime_events.len() <= MAX_EVENTS,
            "Runtime events should be bounded"
        );

        for damage in &observation.damage_memory {
            for layer in &damage.layer_damage {
                assert!(
                    layer.cracked_segments.len() <= 100,
                    "Cracked segments should be bounded"
                );
                assert!(
                    layer.released_segments.len() <= 100,
                    "Released segments should be bounded"
                );
            }
        }
    }

    #[test]
    fn reset_restores_baseline_scene() {
        let mut session = VerticalSliceSession::new().expect("create session");

        let fire_packet = VerticalSliceIngressPacket::fire_test_shot(1, 3);
        session.handle_command(fire_packet).expect("fire shot");

        let reset_packet = VerticalSliceIngressPacket::reset_scene(2);
        let observation = session.handle_command(reset_packet).expect("reset");

        assert!(observation.scene.is_some(), "Scene should be restored");
        assert_eq!(
            observation.damage_memory[0].layer_damage[0].integrity, 1.0,
            "Integrity should be restored to 1.0"
        );
        assert_eq!(
            observation.damage_memory[0].layer_damage[0].accumulated_energy_j, 0.0,
            "Accumulated energy should be reset"
        );
    }

    #[test]
    fn no_mock_backend_in_production_path() {
        let session = VerticalSliceSession::new().expect("create session");
        let type_name = std::any::type_name_of_val(&session);
        assert!(!type_name.contains("Mock"), "Should not use mock backend");
        assert!(!type_name.contains("Fake"), "Should not use fake backend");
    }
}

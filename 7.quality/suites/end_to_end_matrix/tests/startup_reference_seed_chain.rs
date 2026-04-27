//! Startup Reference Seed Chain Tests

#[cfg(test)]
mod tests {
    use stratumx_test_support::*;

    #[test]
    fn test_startup_seed() {
        let world = create_test_world();
        assert!(world.vertical_slice_scene().is_none());
    }

    #[test]
    fn test_reference_seed_bootstrap() {
        let world = create_world_with_scene();
        assert!(world.vertical_slice_scene().is_some());
    }

    #[test]
    fn test_startup_chain() {
        let world = create_test_world();
        let _terrain = create_test_terrain();
        let _sky = create_test_sky();

        assert!(world.vertical_slice_scene().is_none());
    }
}

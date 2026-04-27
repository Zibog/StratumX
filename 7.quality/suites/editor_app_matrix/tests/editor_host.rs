//! Editor Host Tests

#[cfg(test)]
mod tests {
    use stratumx_test_support::*;

    #[test]
    fn test_host_creation() {
        let world = create_test_world();
        assert!(world.vertical_slice_scene().is_none());
    }

    #[test]
    fn test_world_state_access() {
        let world = create_test_world();
        assert!(world.vertical_slice_scene().is_none());
    }
}

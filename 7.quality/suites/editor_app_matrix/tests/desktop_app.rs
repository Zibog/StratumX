//! Desktop App Integration Tests

#[cfg(test)]
mod tests {
    use stratumx_test_support::*;

    #[test]
    fn test_app_initialization() {
        let world = create_test_world();
        assert!(world.vertical_slice_scene().is_none());
    }
}

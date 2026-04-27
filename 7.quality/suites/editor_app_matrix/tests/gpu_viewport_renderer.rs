//! GPU Viewport Renderer Tests

#[cfg(test)]
mod tests {
    use stratumx_test_support::*;

    #[test]
    fn test_renderer_initialization() {
        let terrain = create_test_terrain();
        assert!(terrain.world_size[0] > 0.0);
    }

    #[test]
    fn test_terrain_mesh_generation() {
        let terrain = create_terrain_with_size(256, 256);
        assert_eq!(terrain.world_size[0], 256.0);
    }
}

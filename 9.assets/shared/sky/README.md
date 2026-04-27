# Sky Asset Bundle

Default sky rendering assets for StratumX engine.

## Bundle Structure

- `sky_bundle.json` - Bundle manifest with asset references
- `stars/` - Night sky HDRI textures (16K EXR)
- `moon/` - Moon albedo textures (8K)
- `blue_noise/` - Blue noise textures for dithering (256x256)
- `noise_source/` - HLSL noise generation shaders

## Bundle Registration

The sky bundle is automatically registered in the runtime:
- Path: `9.assets/shared/sky/sky_bundle.json`
- Bundle ID: `default_sky_bundle_v1`
- Registered in: `VerticalSliceScene.sky_bundle_path`

## Assets

### Stars
- `NightSkyHDRI001_16K_HDR.exr` - 16K night sky HDRI for star rendering

### Moon
- `8k_moon.jpg` - 8K moon albedo texture

### Blue Noise
- `bluenoise256.png` - 256x256 blue noise for temporal dithering

### Noise Shaders
- `Common.hlsl` - Common noise utilities
- `ClassicNoise3D.hlsl` - Classic Perlin noise
- `Noise1D.hlsl` - 1D noise functions

## Usage

The bundle path is stored in scene state and accessible via:
```rust
let scene = world.vertical_slice_scene()?;
if let Some(bundle_path) = &scene.sky_bundle_path {
    // Load sky assets from bundle
}
```

## Integration Status

✅ Bundle manifest exists
✅ Asset files present
✅ Registered in VerticalSliceScene
✅ Exposed via SceneDto
✅ Available to editor/runtime

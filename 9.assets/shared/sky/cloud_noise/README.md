# Cloud Noise Volumes

Baked 3D noise textures for volumetric cloud rendering.

## Files

- `perlin_worley_128_rgba.dds` - 128³ RGBA volume, Perlin-Worley noise for cloud shape
- `worley_detail_32_r.dds` - 32³ R volume, Worley noise for cloud detail
- `erosion_3d_32_r.dds` - 32³ R volume, erosion noise for cloud edges
- `curl_2d_128_rg.dds` - 128² RG texture, curl noise for cloud flow

## Generation

These should be generated offline using noise generation tools and baked as DDS volumes.
This is cheaper than computing procedural noise at runtime for every cloud sample.

## Status

Placeholder - needs generation

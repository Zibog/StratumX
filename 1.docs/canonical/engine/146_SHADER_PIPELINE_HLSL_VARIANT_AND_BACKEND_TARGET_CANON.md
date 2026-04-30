# Shader Pipeline HLSL Variant And Backend Target Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define shader source ownership, variant generation, backend targets, and pipeline cache law for photoreal-first rendering.

## Source law
Canonical shader authoring source is HLSL-first.
Material truth is not forked by backend.

## Required targets
- SPIR-V for Vulkan;
- DXIL for Direct3D 12;
- Metal target output for Metal;
- platform-native compiled output for platform ports where legal;
- null validation metadata for headless tests.

## Variant identity
A shader variant key includes:
- shader family;
- material family;
- render feature tier;
- backend target;
- pass class;
- defines/specialization constants;
- vertex layout;
- binding layout;
- debug/capture posture.

## Shader families
- `debug.clear`
- `debug.triangle`
- `debug.grid`
- `terrain.basic`
- `sky.gradient`
- `material.pbr_basic`
- `light.shadow_basic`
- `light.local_basic`
- `post.tonemap_basic`
- `overlay.material_id`

## Tooling obligations
Shader cook must publish:
- source hash;
- target hash;
- backend target;
- compiler diagnostics;
- pipeline key;
- cache hit/miss;
- failure code.

## Editor obligations
Editor must show:
- selected shader variant;
- target output;
- compile status;
- first failure;
- material binding gap;
- fallback variant.

## Current posture
`document_gold / shader_pipeline_defined / implementation_open`


---
# V32 Shader Pipeline Closure

## Required shader families
- `shader.debug_grid`
- `shader.sky_basic`
- `shader.terrain_basic`
- `shader.pbr_opaque`
- `shader.shadow_depth`
- `shader.fullscreen_tonemap`
- `shader.debug_overlay`
- `shader.missing_material`

## Variant key fields
Every variant key must include shader family, backend target, feature tier, material layout id, vertex layout id, lighting mode, shadow mode, output color mode, debug overlay flags, and fallback/degrade rung.

## Reflection output
Shader tooling must emit binding layout, vertex inputs, constants, buffers, textures/samplers, entry points, target profile, compile diagnostics, and cache key.

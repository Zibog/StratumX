# Shader Variant Cook And Pipeline Cache Routing Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define tooling routes for shader variant compilation, backend target generation, pipeline cache population, and failure evidence.

## Required routes
- `route.shader.cook_variant`
- `route.shader.cook_backend_target`
- `route.shader.validate_binding_layout`
- `route.shader.populate_pipeline_cache`
- `route.shader.explain_compile_failure`
- `route.shader.recover_fallback_variant`

## Target outputs
- SPIR-V for Vulkan;
- DXIL for Direct3D 12;
- Metal-target output for Metal;
- platform-native shader output after legal SDK integration;
- null validation metadata.

## Artifact law
Each cook emits:
- source hash;
- target hash;
- backend target;
- compiler diagnostics;
- pipeline key;
- material family;
- feature tier;
- fallback variant id.

## Current posture
`document_gold / shader_tooling_routes_defined / implementation_open`


---
# V32 Tooling Closure: Shader Cook Routes

## Required routes
- `route.shader.cook_variant`
- `route.shader.cook_material_family`
- `route.shader.reflect_variant`
- `route.shader.validate_pipeline_layout`
- `route.shader.pipeline_cache_build`
- `route.shader.failure_report`

## Required artifacts
Compiled shader artifact, reflection packet, pipeline key, material layout compatibility record, compiler diagnostics, cache hit/miss record, and failure packet when failed.

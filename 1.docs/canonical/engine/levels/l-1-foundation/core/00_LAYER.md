# engine_core

## Stack Position

L-1. Foundation

## Primary Role

Minimal engine foundation.

## Canonical Scope

Base math, base types, error model, core contracts, and compile-time feature flags.

## Boundary Rationale

This crate stays at the bottom because every upper crate needs a shared language for types, contracts, errors, and math.

## Canonical Consumers

- `engine_identity`
- `engine_handle`
- `engine_storage_layout`
- `engine_storage_access`
- `engine_storage_mutation`
- `engine_ecs_registry`
- `engine_ecs_query`
- `engine_ecs`
- `engine_world_spatial`
- `engine_world_region`
- `engine_world`
- `engine_material`
- `engine_runtime`
- `engine_kinetics`
- `engine_field`
- `engine_agents`
- `engine_inference`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`
- `engine_content`
- `engine_startup`

## Downward Dependencies

- None.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Math Backbone | Unified math backbone used across the engine. | `40_MATH_BACKBONE.md` |
| Base Types | Core scalar, identifier-adjacent, and structural types. | `41_BASE_TYPES.md` |
| Error Model | Typed error hierarchy for engine contracts. | `42_ERROR_MODEL.md` |
| Core Contracts | Foundational traits and contract surfaces. | `43_CORE_CONTRACTS.md` |
| Feature Flags | Compile-time capability and profile flags. | `44_FEATURE_FLAGS.md` |

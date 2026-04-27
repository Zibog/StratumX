# STRATUMX_FIRST_COMPILEABLE_SKELETON

## 1. Purpose

This document defines the first compileable skeleton of StratumX.
It is part of the authoritative implementation package and may not contradict root, constitutional, or package-closure law.

## 2. Mandatory Crates

- engine_core
- engine_identity
- engine_handle
- engine_storage_layout
- engine_storage_access
- engine_storage_mutation
- engine_ecs_registry
- engine_ecs_query
- engine_ecs
- engine_world_spatial
- engine_world_region
- engine_world
- engine_material
- engine_runtime
- engine_runtime_headless

## 3. Mandatory Compileable Surfaces

- canonical primary public boundary in every mandatory crate;
- minimal config type where the crate owns configurable behavior;
- canonical snapshot/read/apply starter surfaces;
- runtime shell starter surface;
- material lookup starter surface.

## 4. Mandatory Proofs

- workspace compiles;
- no illegal dependency direction;
- no missing primary public boundary;
- no missing canonical starter contracts;
- no implementation-facing surface contradicts queue, budget, or ownership law.

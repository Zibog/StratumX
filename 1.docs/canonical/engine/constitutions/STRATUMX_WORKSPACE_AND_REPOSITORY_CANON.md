# STRATUMX_WORKSPACE_AND_REPOSITORY_CANON

## 1. Purpose

This document defines canonical repository and workspace organization for StratumX.

## 2. Canonical Repository Shape

```text
docs/
    canonical/
        engine/
            root docs
            package closure docs
            constitutions/
            implementation/
            levels/

crates/
    engine_core
    engine_identity
    engine_handle
    engine_storage_layout
    engine_storage_access
    engine_storage_mutation
    engine_ecs_registry
    engine_ecs_query
    engine_ecs
    engine_world_spatial
    engine_world_region
    engine_world
    engine_material
    engine_runtime
    engine_runtime_headless
    engine_runtime_realtime
    engine_stream_control
    engine_residency_control
    engine_memory_control
    engine_transfer_control
    engine_kinetics
    engine_field
    engine_agents
    engine_net_transport
    engine_net_sync
    engine_net_latency
    engine_inference
    engine_generation
    engine_imaging
    engine_acoustics
    engine_content
    engine_startup
```

## 3. Workspace Rules

- one crate = one canonical boundary;
- crate names follow frozen canonical names;
- lower stack crates remain independent and reusable;
- runtime family crates remain separate from startup;
- runtime resource services remain separate from runtime family and from L3 service layers;
- network runtime services remain separate from critical simulation families;
- service layers remain separate from critical simulation families.

## 4. Documentation Rules

- root docs define stack identity, summary maps, and package closure docs;
- constitutions define engine-wide law;
- implementation docs define implementation order, handoff, and starter delivery;
- level docs define local boundaries;
- glossary and alias maps must be updated when canonical terms change.

## 5. Public/Internal Module Law

- public surface must follow canonical crate boundary;
- internal modules may not leak new public family names without naming-freeze updates;
- hidden implementation details may evolve without changing canonical crate identity.

## 6. Startup and Assembly Position

`engine_startup` remains above runtime family, runtime resource services, network runtime services, and service layers.

## 7. Canonical Laws

- workspace shape follows canonical stack;
- repository layout preserves discoverability and non-drift navigation;
- documentation and workspace boundaries evolve together.

## 8. Documentation Packages

- `docs/canonical/engine/` root canon and package closure docs
- `docs/canonical/engine/constitutions/` engine-wide laws
- `docs/canonical/engine/implementation/` implementation-facing engine documents

# STRATUMX_IMPLEMENTATION_ORDER

## 1. Purpose

This document defines the canonical implementation order for StratumX.

## 2. Ordering Law

This order is a topological implementation order over `05_DEPENDENCY_MODEL.md`.
Within each phase, the listed order is authoritative from top to bottom.
A crate may depend on an earlier crate in the same phase, but may not depend on a later crate in the same phase or on any crate in a later phase.

## 3. Canonical Phases

### Phase A
- engine_core
- engine_identity
- engine_handle

### Phase B
- engine_storage_layout
- engine_storage_access
- engine_storage_mutation

### Phase C
- engine_ecs_registry
- engine_ecs_query
- engine_ecs

### Phase D
- engine_world_spatial
- engine_world_region
- engine_world

### Phase E
- engine_material

### Phase F
- engine_runtime
- engine_runtime_headless

### Phase G
- engine_stream_control
- engine_memory_control

### Phase H
- engine_residency_control
- engine_transfer_control

### Phase I
- engine_kinetics

### Phase J
- engine_runtime_realtime

### Phase K
- engine_field
- engine_agents

### Phase L
- engine_net_transport
- engine_net_sync
- engine_net_latency

### Phase M
- engine_inference
- engine_generation
- engine_imaging
- engine_acoustics
- engine_content

### Phase N
- engine_startup

## 4. Handoff Note

`engine_content` is implemented after the mandatory runtime-resource spine because `L1.5` consumes runtime-pack inputs through startup-mediated or externally prepared inputs, not through a direct upward crate dependency into `L3.2`.

## 5. Exit Rule

Every phase exits only when:
- compile gate passes;
- contract gate passes;
- invariant gate passes;
- perf smoke gate passes.

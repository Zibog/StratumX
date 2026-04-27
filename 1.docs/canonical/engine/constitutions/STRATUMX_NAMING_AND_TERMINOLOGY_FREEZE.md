# STRATUMX_NAMING_AND_TERMINOLOGY_FREEZE

## 1. Purpose

This document freezes canonical names and terms for the current StratumX engine canon.

## 2. Frozen Level Names

- L-1. Foundation
- L-0.9. Identity Primitives
- L-0.8. Handle System
- L-0.7. Storage Layout
- L-0.6. Storage Access
- L-0.5. Storage Mutation
- L-0.4. ECS Registry
- L-0.3. ECS Query
- L-0.2. ECS Assembly
- L-0.1. World Spatial
- L-0.05. World Region
- L0. World Truth
- L0.5. Shared World Property Substrate
- L1. Runtime Kernel
- L1.5. Runtime Resource Services
- L2. Critical Simulation Families
- L2.5. Network Runtime Services
- L3.0. Model Systems
- L3.1. Synthesis Systems
- L3.2. Resource Systems
- L4. Startup

## 3. Frozen Crate Names

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
- engine_runtime_realtime
- engine_stream_control
- engine_residency_control
- engine_memory_control
- engine_transfer_control
- engine_kinetics
- engine_field
- engine_agents
- engine_net_transport
- engine_net_sync
- engine_net_latency
- engine_inference
- engine_generation
- engine_imaging
- engine_acoustics
- engine_content
- engine_startup

## 4. Frozen Umbrella Terms

- lower stack = L-1..L0
- runtime family = engine_runtime + engine_runtime_headless + engine_runtime_realtime
- runtime resource services = engine_stream_control + engine_residency_control + engine_memory_control + engine_transfer_control
- network runtime services = engine_net_transport + engine_net_sync + engine_net_latency
- service layers = L3.0 + L3.1 + L3.2
- world instance = one authoritative world plus one active runtime authority
- runtime authority = the active runtime family instance that legally progresses one world instance

## 4.1. Frozen Performance Terms

- critical execution lane
- performance-critical substrate
- traversal plan
- dense execution handle
- simulation tier
- segmented apply
- residency hysteresis
- quantization class
- rewindable domain
- visibility hierarchy
- replay-safe envelope

## 4.2. Frozen Benchmark Terms

- benchmark-common-cpu

## 5. Canonical Alias Discipline

Historical names may remain only in alias, transition, and preservation maps.

## 6. Law

No document may introduce a competing stack name, crate name, or umbrella term without updating:

- `02_CANONICAL_STACK.md`
- `03_ROLE_MAP.md`
- `09_GLOSSARY.md`
- `13_ALIAS_AND_RENAME_MAP.md`

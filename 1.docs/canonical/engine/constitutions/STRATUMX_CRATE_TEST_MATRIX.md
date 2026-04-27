# STRATUMX_CRATE_TEST_MATRIX

## 1. Purpose

This document defines mandatory crate-by-crate test obligations.

## 2. Matrix

| Crate | Mandatory Core Test Classes | Mandatory Operational Subclasses | Hard Invariants |
|---|---|---|---|
| engine_core | structural, contract, invariants, performance smoke | benchmark gate | stable base types, typed errors, and core contract integrity |
| engine_identity | structural, contract, invariants, property, performance smoke | benchmark gate | identity uniqueness, identity stability |
| engine_handle | structural, contract, invariants, determinism, performance smoke | benchmark gate | stale handle rejection, validation correctness |
| engine_storage_layout | structural, contract, invariants, performance smoke | benchmark gate | canonical layout descriptors remain legal |
| engine_storage_access | structural, contract, invariants, concurrency legality, performance smoke | benchmark gate | legal read/write separation |
| engine_storage_mutation | structural, contract, invariants, concurrency legality, performance smoke | benchmark gate | staged mutation legality |
| engine_ecs_registry | structural, contract, invariants, property, performance smoke | benchmark gate | structural registration truth |
| engine_ecs_query | structural, contract, invariants, determinism, performance smoke | benchmark gate | deterministic traversal legality |
| engine_ecs | structural, contract, invariants, determinism, performance smoke | benchmark gate | ECS façade legality |
| engine_world_spatial | structural, contract, invariants, performance smoke | benchmark gate | spatial descriptor legality |
| engine_world_region | structural, contract, invariants, performance smoke | benchmark gate | region/chunk legality |
| engine_world | structural, contract, invariants, determinism, concurrency legality, performance smoke | benchmark gate | authoritative apply boundary, snapshot consistency |
| engine_material | structural, contract, invariants, concurrency legality, performance smoke | benchmark gate | immutable shared property truth |
| engine_runtime | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | degradation, benchmark gate | one active runtime authority, phase legality, apply legality |
| engine_runtime_headless | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | degradation, benchmark gate | headless profile parity with runtime law |
| engine_runtime_realtime | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | degradation, benchmark gate | realtime profile parity with runtime law |
| engine_stream_control | structural, contract, invariants, concurrency legality, performance smoke, fault and degradation | pressure, queue, benchmark gate | no illegal activation/eviction authority leakage |
| engine_residency_control | structural, contract, invariants, concurrency legality, performance smoke, fault and degradation | pressure, residency, benchmark gate | no illegal residency truth leakage |
| engine_memory_control | structural, contract, invariants, concurrency legality, performance smoke, fault and degradation | pressure, allocator, benchmark gate | allocator/lifetime ownership remains local |
| engine_transfer_control | structural, contract, invariants, concurrency legality, performance smoke, fault and degradation | transfer, fence, benchmark gate | transfer ownership remains local |
| engine_kinetics | structural, contract, invariants, determinism, concurrency legality, property, performance smoke | benchmark gate | legal motion/contact deltas only |
| engine_field | structural, contract, invariants, determinism, concurrency legality, property, performance smoke | benchmark gate | legal distributed field deltas only |
| engine_agents | structural, contract, invariants, determinism, concurrency legality, property, performance smoke | benchmark gate | legal agent deltas only |
| engine_net_transport | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | role legality, benchmark gate | ordered publication where required, no hidden authority |
| engine_net_sync | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | role legality, compatibility rejection, benchmark gate | server authority preserved, sync outputs remain staged/exported only |
| engine_net_latency | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | role legality, benchmark gate | prediction/reconcile/rewind remain non-authoritative |
| engine_inference | structural, contract, invariants, fault and degradation, performance smoke | degradation, enablement legality, compatibility rejection, benchmark gate | request/result legality, no world authority |
| engine_generation | structural, contract, invariants, fault and degradation, performance smoke | degradation, enablement legality, compatibility rejection, benchmark gate | request/result legality, no world authority |
| engine_imaging | structural, contract, invariants, fault and degradation, performance smoke | degradation, enablement legality, compatibility rejection, benchmark gate | synthesis result legality |
| engine_acoustics | structural, contract, invariants, fault and degradation, performance smoke | degradation, enablement legality, compatibility rejection, benchmark gate | synthesis result legality |
| engine_content | structural, contract, invariants, fault and degradation, performance smoke | degradation, enablement legality, compatibility rejection, benchmark gate | resource pipeline legality |
| engine_startup | structural, contract, invariants, performance smoke | restoration legality, compatibility rejection, enablement legality, benchmark gate | legal assembly decisions, legal restoration decisions |

## 3. Canonical Rule

Crate-level implementation and CI planning must derive from this matrix and may only add tests, not remove mandatory classes or subclasses.

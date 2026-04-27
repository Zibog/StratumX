# STRATUMX_CRATE_PERFORMANCE_BUDGETS

## 1. Purpose

This document freezes planning envelopes for canonical crates.
A crate that repeatedly breaches its own envelope is illegal even when the whole-engine profile ceiling is not yet crossed.

## 2. Canonical Planning Envelopes

| Crate | Canonical planning envelope | Notes |
|---|---|---|
| engine_core | <= 0.05 ms p95 per 100,000 hot type ops; zero steady-state allocations | foundation only |
| engine_identity | <= 0.12 ms p95 per 10,000 id lookups/creations | generation-safe only |
| engine_handle | <= 0.10 ms p95 per 10,000 validations | stable/public only |
| engine_storage_layout | <= 0.20 ms p95 per 10,000 descriptor assemblies | compiled layouts only |
| engine_storage_access | <= 0.20 ms p95 per 10,000 traversal-entry binds | zero avoidable heap churn |
| engine_storage_mutation | <= 0.30 ms p95 per 10,000 staged records assembled | batch-oriented only |
| engine_ecs_registry | <= 0.25 ms p95 per 10,000 membership changes | never owns steady-state traversal |
| engine_ecs_query | <= 0.35 ms p95 per 10,000 hot-query descriptor compiles; zero compiles on cache hit | traversal-plan cache mandatory |
| engine_ecs | <= 0.30 ms p95 / <= 0.45 ms p99 facade overhead per authoritative step on canonical lower-stack fixture | assembly only |
| engine_world_spatial | <= 0.10 ms p95 per 100,000 address reads | local/region addressing only |
| engine_world_region | <= 0.20 ms p95 per 10,000 dirty or migration metadata ops | bounded region metadata |
| engine_world | snapshot build <= 1.00 ms p95 / 1.50 ms p99 and apply integration orchestration <= 1.50 ms p95 / 2.25 ms p99 and aggregate world-owned overhead <= 2.25 ms p95 / 3.25 ms p99 on `world-apply-a` | segmented apply only |
| engine_material | <= 0.08 ms p95 per 100,000 hot lookups | flattened immutable lookup |
| engine_runtime | <= 0.75 ms p95 and <= 1.25 ms p99 per tick on realtime profiles | runtime kernel ceiling |
| engine_runtime_headless | <= 0.10 ms p95 profile-shell overhead per tick | shell only |
| engine_runtime_realtime | <= 0.10 ms p95 profile-shell overhead per frame | shell only |
| engine_stream_control | <= 0.50 ms p95 activation/scheduling overhead per frame | inside `L1.5` ceiling |
| engine_residency_control | <= 0.35 ms p95 accounting/hysteresis overhead per frame | inside `L1.5` ceiling |
| engine_memory_control | <= 0.25 ms p95 allocator reset/trim overhead per cycle | inside `L1.5` ceiling |
| engine_transfer_control | <= 0.75 ms p95 transfer-overhead CPU on realtime profiles | decode/stage/upload bounded |
| engine_kinetics | <= 3.50 ms p95 / <= 5.00 ms p99 per authoritative step on `destruction-heavy-a` | absolute family ceiling |
| engine_field | <= 4.00 ms p95 / <= 5.50 ms p99 per authoritative step on `destruction-heavy-a` | absolute family ceiling |
| engine_agents | <= 4.00 ms p95 / <= 5.50 ms p99 per authoritative step on `agent-cycle-a` | absolute family ceiling |
| engine_net_transport | <= 0.30 ms p95 per active peer for encode/decode/session overhead | transport state bounded |
| engine_net_sync | <= 0.40 ms p95 per active peer for publication/ack work | interest- and quantization-driven |
| engine_net_latency | <= 0.25 ms p95 per active peer for predict/reconcile/history overhead | short bounded history only |
| engine_inference | explicit service budget only; <= 0.10 ms p95 normalization overhead per request | async only |
| engine_generation | explicit service budget only; <= 0.10 ms p95 normalization overhead per output | async only |
| engine_imaging | extraction + resource residency + upload staging + frame resource policy <= 2.25 ms p95 / <= 3.25 ms p99 on realtime profiles | transfer-separated |
| engine_acoustics | propagation + reflection/occlusion + runtime buffering <= 0.90 ms p95 / <= 1.30 ms p99 on realtime profiles | pressure-aware |
| engine_content | off-cadence cook/pack transforms only; <= 4.00 ms p95 per canonical page-transform unit | resource layer only |
| engine_startup | launch-time validation and warmup must fit startup budgets and may not leak post-launch ownership | startup only |

## 3. Rule

A crate that repeatedly breaches its planning envelope is not “close enough” merely because the whole-engine ceiling is not yet crossed.

## 4. Composition Rule

Per-crate envelopes are legal only when they also fit the integrated profile envelopes frozen by `STRATUMX_PROFILE_COMPOSITION_PROOF.md` and the end-to-end mapping frozen by `STRATUMX_ENGINE_BUDGET_LEDGER.md`.

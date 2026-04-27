# STRATUMX_PERFORMANCE_GATE_MATRIX

## 1. Purpose

This document freezes the benchmark gates that back performance legality.
Regression gates are legal only against locked baselines registered in the locked baseline table; absolute gates are legal only against frozen hard ceilings.

## 2. Gate Rule

- a row with `Gate Kind = regression` must cite a locked baseline id owned by the same profile floor or by an explicitly legal `benchmark-common-cpu` row;
- a row with `Gate Kind = absolute` must cite `ABSOLUTE-GATE` only;
- a profile-specific row may not cite a baseline captured on another profile floor;
- when a benchmark has both regression and hard-ceiling meaning, the regression row binds the locked baseline and the hard ceiling remains enforced by the owning budget class or explicit absolute row;
- diagnostics and degradation laws are performance-relevant and therefore require explicit gate rows.
- every crate marked `benchmark gate` by `STRATUMX_CRATE_TEST_MATRIX.md` and budgeted by `STRATUMX_CRATE_PERFORMANCE_BUDGETS.md` must resolve to at least one active gate row in the matrix below.

## 3. Gate Matrix

| Benchmark scope | Fixture ID | Baseline ID | Gate kind | Hard gate | Gold gate | Failure class | Severity |
|---|---|---|---|---|---|---|---|
| engine_core hot type ops | `lower-stack-traversal-a` | `BL-LS-CORE-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_identity id lookup / creation | `lower-stack-traversal-a` | `BL-LS-ID-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_handle validation | `lower-stack-traversal-a` | `BL-LS-HANDLE-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_storage_layout descriptor assemblies | `lower-stack-traversal-a` | `BL-LS-LAYOUT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_storage_access traversal-entry bind | `lower-stack-traversal-a` | `BL-LS-ACCESS-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_ecs_registry membership update/read | `lower-stack-traversal-a` | `BL-LS-REG-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_ecs_query traversal | `lower-stack-traversal-a` | `BL-LS-QUERY-01` | regression | throughput may not fall below 95% of locked baseline | throughput >= 98% of locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_ecs facade overhead | `lower-stack-traversal-a` | `BL-LS-ECS-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_world_spatial descriptor and address reads | `lower-stack-traversal-a` | `BL-LS-SPATIAL-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_world_region region metadata and dirty tracking | `lower-stack-traversal-a` | `BL-LS-REGION-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_storage_mutation staged record assembly | `world-apply-a` | `BL-STORAGE-MUT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_material immutable lookup | `lower-stack-traversal-a` | `BL-LS-MAT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime empty tick | `runtime-kernel-a` | `BL-RT-EMPTY-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime minimal tick | `runtime-kernel-a` | `BL-RT-MIN-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime orchestration | `runtime-kernel-a` | `BL-RT-ORCH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime contention | `runtime-kernel-a` | `BL-RT-CONT-01` | regression | blocked-worker ratio may not exceed locked baseline by > 5% | blocked-worker ratio <= locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_stream_control activation/prefetch pass (interactive) | `streaming-locality-a` | `BL-SVC-STREAM-INT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_residency_control residency accounting (interactive) | `streaming-locality-a` | `BL-SVC-RESID-INT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_memory_control frame reset and pool trim | `streaming-locality-a` | `BL-SVC-MEM-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_transfer_control decode/staging/upload chain (interactive) | `streaming-locality-a` | `BL-SVC-XFER-INT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_stream_control activation/prefetch pass (listen-host) | `streaming-locality-a` | `BL-SVC-STREAM-LH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_residency_control residency accounting (listen-host) | `streaming-locality-a` | `BL-SVC-RESID-LH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_memory_control frame reset and pool trim (listen-host) | `streaming-locality-a` | `BL-SVC-MEM-LH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_transfer_control decode/staging/upload chain (listen-host) | `streaming-locality-a` | `BL-SVC-XFER-LH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_stream_control activation/prefetch pass (headless) | `streaming-locality-a` | `BL-SVC-STREAM-HD-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_residency_control residency accounting (headless) | `streaming-locality-a` | `BL-SVC-RESID-HD-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_memory_control frame reset and pool trim (headless) | `streaming-locality-a` | `BL-SVC-MEM-HD-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_transfer_control decode/staging/apply chain (headless) | `streaming-locality-a` | `BL-SVC-XFER-HD-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime queue depth stability | `runtime-kernel-a` | `ABSOLUTE-GATE` | absolute | no queue exceeds hard ceiling or flush-age ceiling | <= 80% of hard ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| simulation tier near-footprint envelope (listen-host) | `world-apply-a` | `ABSOLUTE-GATE` | absolute | Tier N footprint stays within hard ceilings | <= 80% of Tier N ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| simulation tier near-footprint envelope (headless) | `world-apply-a` | `ABSOLUTE-GATE` | absolute | Tier N footprint stays within hard ceilings | <= 80% of Tier N ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_headless whole-profile progression | `world-apply-a` | `BL-HEADLESS-01` | regression | parallel efficiency must not fall below 95% of locked baseline | parallel efficiency >= 98% of locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime whole-profile progression (interactive) | `mixed-pressure-realtime-a` | `BL-REALTIME-INT-01` | regression | jitter must not exceed 5% over locked baseline | jitter <= 2% over locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime whole-profile progression (listen-host) | `mixed-pressure-realtime-a` | `BL-REALTIME-LH-01` | regression | jitter must not exceed 5% over locked baseline | jitter <= 2% over locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime low-latency queue depth (interactive) | `mixed-pressure-realtime-a` | `ABSOLUTE-GATE` | absolute | queued frame count and queue age may not exceed hard ceilings | <= 80% of hard ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime low-latency queue depth (listen-host) | `mixed-pressure-realtime-a` | `ABSOLUTE-GATE` | absolute | queued frame count and queue age may not exceed hard ceilings | <= 80% of hard ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime mixed-pressure envelope (interactive) | `mixed-pressure-realtime-a` | `BL-MIXED-REALTIME-INT-01` | regression | frame p95 must not exceed locked baseline by > 5% and may not cross any hard ceiling | frame p95 <= baseline and all hard ceilings <= 80% | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_realtime mixed-pressure envelope (listen-host) | `mixed-pressure-realtime-a` | `BL-MIXED-REALTIME-LH-01` | regression | frame p95 must not exceed locked baseline by > 5% and may not cross any hard ceiling | frame p95 <= baseline and all hard ceilings <= 80% | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_runtime_headless mixed-pressure envelope | `mixed-pressure-headless-a` | `BL-MIXED-HEADLESS-01` | regression | tick p95 must not exceed locked baseline by > 5% and may not cross any hard ceiling | tick p95 <= baseline and all hard ceilings <= 80% | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_world snapshot build | `world-apply-a` | `ABSOLUTE-GATE` | absolute | <= hard ceiling from absolute budget constitution | <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_world apply batch integration | `world-apply-a` | `ABSOLUTE-GATE` | absolute | <= hard ceiling from absolute budget constitution | <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_transport packet encode/decode (listen-host) | `network-listen-host-a` | `BL-NET-TX-LH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_sync snapshot/delta publication (listen-host) | `network-listen-host-a` | `BL-NET-SYNC-LH-01` | regression | <= 5% regression and per-client budget stays within hard ceiling | <= 2% regression and <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_sync ack backlog and delivery state (listen-host) | `network-listen-host-a` | `ABSOLUTE-GATE` | absolute | ack and reliable backlog stay within hard ceilings | <= 80% of hard ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_latency predict/reconcile/rewind (listen-host) | `network-listen-host-a` | `BL-NET-LAT-LH-01` | regression | <= 5% regression and rewind stays within hard ceiling | <= 2% regression and <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_transport packet encode/decode (headless) | `network-headless-a` | `BL-NET-TX-HD-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_sync snapshot/delta publication (headless) | `network-headless-a` | `BL-NET-SYNC-HD-01` | regression | <= 5% regression and per-client budget stays within hard ceiling | <= 2% regression and <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_sync ack backlog and delivery state (headless) | `network-headless-a` | `ABSOLUTE-GATE` | absolute | ack and reliable backlog stay within hard ceilings | <= 80% of hard ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_net_latency predict/reconcile/rewind (headless) | `network-headless-a` | `BL-NET-LAT-HD-01` | regression | <= 5% regression and rewind stays within hard ceiling | <= 2% regression and <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_kinetics batch compute | `destruction-heavy-a` | `BL-KIN-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_field regional solve | `destruction-heavy-a` | `BL-FIELD-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_agents batch cycle | `agent-cycle-a` | `BL-AGENT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_imaging extraction baseline (interactive) | `visibility-realtime-a` | `BL-IMG-INT-01` | regression | <= 5% regression and extraction stays within hard ceiling | <= 2% regression and <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_imaging extraction baseline (listen-host) | `visibility-realtime-a` | `BL-IMG-LH-01` | regression | <= 5% regression and extraction stays within hard ceiling | <= 2% regression and <= 80% of hard ceiling | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_acoustics solve baseline (interactive) | `acoustics-streaming-a` | `BL-AUDIO-INT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression vs locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_acoustics solve baseline (listen-host) | `acoustics-streaming-a` | `BL-AUDIO-LH-01` | regression | <= 5% regression vs locked baseline | <= 2% regression vs locked baseline | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_inference output shaping | `service-normalization-a` | `BL-MODEL-INF-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_generation package shaping | `service-normalization-a` | `BL-MODEL-GEN-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_content runtime page transform | `content-transform-a` | `BL-CONTENT-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| engine_startup legal assembly launch path | `startup-assembly-a` | `BL-STARTUP-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| deterministic replay checkpoint cadence | `replay-determinism-a` | `ABSOLUTE-GATE` | absolute | cadence drift == 0 and checkpoint/window ceilings stay legal | same as hard gate | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| deterministic replay checkpoint compare | `replay-determinism-a` | `BL-REPLAY-CMP-01` | regression | <= 5% regression vs locked baseline | <= 2% regression | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| diagnostics ceiling probe | `mixed-pressure-realtime-a` | `ABSOLUTE-GATE` | absolute | diagnostics ceilings may not be crossed | <= 80% of diagnostics ceilings | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |
| degradation-order integrity probe | `mixed-pressure-realtime-a` | `ABSOLUTE-GATE` | absolute | degradation order may not invert under pressure | no inversions and all degraded classes remain legal | FAIL_PERFORMANCE | B3_RELEASE_BLOCK |

## 4. Profile-Seal Closure Matrix

| Gate family | Profile | Binding posture | Seal result |
|---|---|---|---|
| runtime + realtime shell | `interactive-60` | same-floor regression rows plus absolute queue row | sealed |
| runtime + realtime shell | `listen-host-60` | same-floor regression rows plus absolute queue row | sealed |
| runtime + headless shell | `headless-20` | same-floor regression rows only | sealed |
| `L1.5` services | `interactive-60` | same-floor rows plus legal common memory row and absolute queue row | sealed |
| `L1.5` services | `listen-host-60` | same-floor rows plus absolute queue row | sealed |
| `L1.5` services | `headless-20` | same-floor rows plus absolute queue row | sealed |
| `L2.5` network runtime | `listen-host-60` | same-floor regression rows plus absolute backlog row | sealed |
| `L2.5` network runtime | `headless-20` | same-floor regression rows plus absolute backlog row | sealed |
| imaging | `interactive-60` | same-floor extraction row + same-floor mixed envelope row | sealed |
| imaging | `listen-host-60` | same-floor extraction row + same-floor mixed envelope row | sealed |
| acoustics | `interactive-60` | same-floor solve row + same-floor mixed envelope row | sealed |
| acoustics | `listen-host-60` | same-floor solve row + same-floor mixed envelope row | sealed |
| mandatory simulation families | `listen-host-60` | same-floor regression rows plus absolute tier row | sealed |
| mandatory simulation families | `headless-20` | same-floor mixed envelope row plus absolute tier row | sealed |

## 5. Rule

A performance gate package is illegal when any absolute row cites a locked baseline id, when any profile-specific regression row cites another profile floor, or when any gate family above loses closure.

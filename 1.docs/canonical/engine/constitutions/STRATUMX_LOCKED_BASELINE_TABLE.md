# STRATUMX_LOCKED_BASELINE_TABLE

## 1. Purpose

This document freezes the locked performance baselines that back regression-gate legality.
A locked baseline is a same-profile benchmark binding unless it is explicitly legal as `benchmark-common-cpu`.

## 2. Rule

- a profile may not reuse a baseline row captured on a different profile floor unless the row is explicitly marked `benchmark-common-cpu`;
- `benchmark-common-cpu` is legal only for CPU-only rows whose scope excludes presentation, GPU timing, backend-sensitive extraction, and role-owned network burden;
- budget-ledger and profile-composition bindings must resolve to baseline ids owned by the same profile floor or to explicitly legal `benchmark-common-cpu` rows only;
- every locked baseline row must bind exactly one capture-sheet id, one capture-result id, one hardware floor id, one protocol id, and one fixture id;
- absolute gates may cite `ABSOLUTE-GATE` instead of a baseline id;
- when a surface is converted to an absolute gate, stale locked baseline registrations for that exact surface must be removed in the same canon revision.

## 3. Locked Baselines

| Baseline ID | Fixture ID | Scope | Capture Profile | Hardware Floor ID | Backend ID | Capture Sheet ID | Capture Result ID | Protocol ID | Canonical baseline |
|---|---|---|---|---|---|---|---|---|---|
| `BL-LS-CORE-01` | `lower-stack-traversal-a` | `engine_core hot type ops` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.045 ms per 100,000 ops |
| `BL-LS-ID-01` | `lower-stack-traversal-a` | `engine_identity id lookup / creation` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.42 ms per 10,000 ops |
| `BL-LS-HANDLE-01` | `lower-stack-traversal-a` | `engine_handle validation` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.28 ms per 10,000 ops |
| `BL-LS-LAYOUT-01` | `lower-stack-traversal-a` | `engine_storage_layout descriptor assemblies` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.17 ms per 10,000 ops |
| `BL-LS-ACCESS-01` | `lower-stack-traversal-a` | `engine_storage_access traversal-entry bind` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.18 ms per 10,000 ops |
| `BL-LS-REG-01` | `lower-stack-traversal-a` | `engine_ecs_registry membership update/read` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.31 ms per 10,000 ops |
| `BL-LS-QUERY-01` | `lower-stack-traversal-a` | `engine_ecs_query traversal` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | >= 14.0 M traversed items/s |
| `BL-LS-ECS-01` | `lower-stack-traversal-a` | `engine_ecs facade overhead` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.18 ms per 10,000 facade ops |
| `BL-LS-SPATIAL-01` | `lower-stack-traversal-a` | `engine_world_spatial descriptor and address reads` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.06 ms per 100,000 reads |
| `BL-LS-REGION-01` | `lower-stack-traversal-a` | `engine_world_region region metadata and dirty tracking` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.14 ms per 10,000 ops |
| `BL-STORAGE-MUT-01` | `world-apply-a` | `engine_storage_mutation staged record assembly` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.26 ms per 10,000 staged records |
| `BL-LS-MAT-01` | `lower-stack-traversal-a` | `engine_material immutable lookup` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.05 ms per 100,000 lookups |
| `BL-RT-EMPTY-01` | `runtime-kernel-a` | `engine_runtime empty tick` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 12 us |
| `BL-RT-MIN-01` | `runtime-kernel-a` | `engine_runtime minimal tick` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 22 us |
| `BL-RT-ORCH-01` | `runtime-kernel-a` | `engine_runtime orchestration` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.48 ms |
| `BL-RT-CONT-01` | `runtime-kernel-a` | `engine_runtime contention` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | blocked-worker ratio <= 0.5% |
| `BL-SVC-STREAM-INT-01` | `streaming-locality-a` | `engine_stream_control activation/prefetch pass` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.30 ms |
| `BL-SVC-RESID-INT-01` | `streaming-locality-a` | `engine_residency_control residency accounting` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.22 ms |
| `BL-SVC-MEM-01` | `streaming-locality-a` | `engine_memory_control frame reset and pool trim` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.12 ms |
| `BL-SVC-XFER-INT-01` | `streaming-locality-a` | `engine_transfer_control decode/staging/upload chain` | `interactive-60` | `HF-INT-60-01` | `VK-RT-01` | `CS-INT-60-VK-01` | `CR-INT-60-VK-01` | `BENCH-PROTO-v2` | p95 <= 0.75 ms |
| `BL-SVC-STREAM-LH-01` | `streaming-locality-a` | `engine_stream_control activation/prefetch pass` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.35 ms |
| `BL-SVC-RESID-LH-01` | `streaming-locality-a` | `engine_residency_control residency accounting` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.28 ms |
| `BL-SVC-MEM-LH-01` | `streaming-locality-a` | `engine_memory_control frame reset and pool trim` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.14 ms |
| `BL-SVC-XFER-LH-01` | `streaming-locality-a` | `engine_transfer_control decode/staging/upload chain` | `listen-host-60` | `HF-LH-60-01` | `VK-RT-01` | `CS-LH-60-VK-01` | `CR-LH-60-VK-01` | `BENCH-PROTO-v2` | p95 <= 0.90 ms |
| `BL-SVC-STREAM-HD-01` | `streaming-locality-a` | `engine_stream_control activation/prefetch pass` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.42 ms |
| `BL-SVC-RESID-HD-01` | `streaming-locality-a` | `engine_residency_control residency accounting` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.32 ms |
| `BL-SVC-MEM-HD-01` | `streaming-locality-a` | `engine_memory_control frame reset and pool trim` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.16 ms |
| `BL-SVC-XFER-HD-01` | `streaming-locality-a` | `engine_transfer_control decode/staging/apply chain` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.45 ms |
| `BL-HEADLESS-01` | `world-apply-a` | `engine_runtime_headless whole-profile progression` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | parallel efficiency >= 0.88 |
| `BL-REALTIME-INT-01` | `mixed-pressure-realtime-a` | `engine_runtime_realtime whole-profile progression` | `interactive-60` | `HF-INT-60-01` | `VK-RT-01` | `CS-INT-60-VK-01` | `CR-INT-60-VK-01` | `BENCH-PROTO-v2` | frame jitter <= 1.4% |
| `BL-REALTIME-LH-01` | `mixed-pressure-realtime-a` | `engine_runtime_realtime whole-profile progression` | `listen-host-60` | `HF-LH-60-01` | `VK-RT-01` | `CS-LH-60-VK-01` | `CR-LH-60-VK-01` | `BENCH-PROTO-v2` | frame jitter <= 1.8% |
| `BL-MIXED-REALTIME-INT-01` | `mixed-pressure-realtime-a` | `mixed pressure integrated realtime progression` | `interactive-60` | `HF-INT-60-01` | `VK-RT-01` | `CS-INT-60-VK-01` | `CR-INT-60-VK-01` | `BENCH-PROTO-v2` | p95 frame time <= 13.5 ms and all tracked classes <= 80% of hard ceiling |
| `BL-MIXED-REALTIME-LH-01` | `mixed-pressure-realtime-a` | `mixed pressure integrated realtime progression` | `listen-host-60` | `HF-LH-60-01` | `VK-RT-01` | `CS-LH-60-VK-01` | `CR-LH-60-VK-01` | `BENCH-PROTO-v2` | p95 frame time <= 13.5 ms and all tracked classes <= 80% of hard ceiling |
| `BL-MIXED-HEADLESS-01` | `mixed-pressure-headless-a` | `mixed pressure integrated headless progression` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 tick time <= 40.0 ms and all tracked classes <= 80% of hard ceiling |
| `BL-NET-TX-LH-01` | `network-listen-host-a` | `engine_net_transport packet encode/decode` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.19 ms per peer |
| `BL-NET-SYNC-LH-01` | `network-listen-host-a` | `engine_net_sync snapshot/delta publication` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.28 ms per peer |
| `BL-NET-LAT-LH-01` | `network-listen-host-a` | `engine_net_latency predict/reconcile/rewind` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.16 ms per peer |
| `BL-NET-TX-HD-01` | `network-headless-a` | `engine_net_transport packet encode/decode` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.21 ms per peer |
| `BL-NET-SYNC-HD-01` | `network-headless-a` | `engine_net_sync snapshot/delta publication` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.30 ms per peer |
| `BL-NET-LAT-HD-01` | `network-headless-a` | `engine_net_latency predict/reconcile/rewind` | `headless-20` | `HF-HD-20-01` | `CPU-ONLY` | `CS-HD-20-CPU-01` | `CR-HD-20-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.18 ms per peer |
| `BL-KIN-01` | `destruction-heavy-a` | `engine_kinetics batch compute` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 2.40 ms |
| `BL-FIELD-01` | `destruction-heavy-a` | `engine_field regional solve` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 2.80 ms |
| `BL-AGENT-01` | `agent-cycle-a` | `engine_agents batch cycle` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 2.10 ms |
| `BL-IMG-INT-01` | `visibility-realtime-a` | `engine_imaging extraction baseline` | `interactive-60` | `HF-INT-60-01` | `VK-RT-01` | `CS-INT-60-VK-01` | `CR-INT-60-VK-01` | `BENCH-PROTO-v2` | p95 <= 0.90 ms |
| `BL-IMG-LH-01` | `visibility-realtime-a` | `engine_imaging extraction baseline` | `listen-host-60` | `HF-LH-60-01` | `VK-RT-01` | `CS-LH-60-VK-01` | `CR-LH-60-VK-01` | `BENCH-PROTO-v2` | p95 <= 0.95 ms |
| `BL-AUDIO-INT-01` | `acoustics-streaming-a` | `engine_acoustics solve baseline` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.65 ms |
| `BL-AUDIO-LH-01` | `acoustics-streaming-a` | `engine_acoustics solve baseline` | `listen-host-60` | `HF-LH-60-01` | `CPU-ONLY` | `CS-LH-60-CPU-01` | `CR-LH-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.72 ms |
| `BL-MODEL-INF-01` | `service-normalization-a` | `engine_inference output shaping` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.25 ms |
| `BL-MODEL-GEN-01` | `service-normalization-a` | `engine_generation package shaping` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 0.30 ms |
| `BL-CONTENT-01` | `content-transform-a` | `engine_content runtime page transform` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 3.40 ms per canonical page-transform unit |
| `BL-STARTUP-01` | `startup-assembly-a` | `engine_startup legal assembly launch path` | `interactive-60` | `HF-INT-60-01` | `CPU-ONLY` | `CS-INT-60-CPU-01` | `CR-INT-60-CPU-01` | `BENCH-PROTO-v2` | p95 <= 180 ms warm start |
| `BL-REPLAY-CMP-01` | `replay-determinism-a` | `deterministic replay checkpoint compare` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | `CPU-ONLY` | `CS-CMN-CPU-01` | `CR-CMN-CPU-01` | `BENCH-PROTO-v2` | p95 <= 1.4 ms |

## 4. Profile-Seal Audit

| Baseline ID | Capture Profile | Hardware Floor ID | Profile-seal status | Seal note |
|---|---|---|---|---|
| `BL-LS-CORE-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-ID-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-HANDLE-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-LAYOUT-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-ACCESS-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-REG-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-QUERY-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-ECS-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-SPATIAL-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-REGION-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-STORAGE-MUT-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-LS-MAT-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-RT-EMPTY-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-RT-MIN-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-RT-ORCH-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-RT-CONT-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-SVC-STREAM-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-SVC-RESID-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-SVC-MEM-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-SVC-XFER-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-SVC-STREAM-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-SVC-RESID-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-SVC-MEM-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-SVC-XFER-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-SVC-STREAM-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-SVC-RESID-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-SVC-MEM-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-SVC-XFER-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-HEADLESS-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-REALTIME-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-REALTIME-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-MIXED-REALTIME-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-MIXED-REALTIME-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-MIXED-HEADLESS-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-NET-TX-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-NET-SYNC-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-NET-LAT-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-NET-TX-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-NET-SYNC-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-NET-LAT-HD-01` | `headless-20` | `HF-HD-20-01` | sealed | same-profile floor binding |
| `BL-KIN-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-FIELD-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-AGENT-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-IMG-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-IMG-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-AUDIO-INT-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-AUDIO-LH-01` | `listen-host-60` | `HF-LH-60-01` | sealed | same-profile floor binding |
| `BL-MODEL-INF-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-MODEL-GEN-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-CONTENT-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |
| `BL-STARTUP-01` | `interactive-60` | `HF-INT-60-01` | sealed | same-profile floor binding |
| `BL-REPLAY-CMP-01` | `benchmark-common-cpu` | `HF-CMN-CPU-01` | sealed | common CPU row legal across profiles only because scope excludes presentation, GPU timing, and host-role burden |

## 5. Stack Binding

This baseline table is bound to stack marker `SX-CANON/1.0.24/STACK-v30`.
A later stack marker invalidates the rows until a new locked table is registered.

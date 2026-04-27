# STRATUMX_PROFILE_COMPOSITION_PROOF

## 1. Purpose

This document freezes the integrated composition proof for canonical profile legality.
It is the mechanical bridge between budget classes, gate rows, locked baselines or absolute gates, capture artifacts, and profile envelopes.

## 2. Proof Rule

This package proves binding closure, not whole-document semantic completeness.
Mechanical profile sealing exists only when every cited budget class resolves through:
- a registered gate row in `STRATUMX_PERFORMANCE_GATE_MATRIX.md`;
- a same-profile locked baseline row or `ABSOLUTE-GATE` in `STRATUMX_LOCKED_BASELINE_TABLE.md`;
- a registered capture sheet/result pair in `STRATUMX_BASELINE_CAPTURE_SHEETS.md` and `STRATUMX_BASELINE_CAPTURE_RESULTS.md` when the row is regression-backed;
- a numeric-ledger row in `STRATUMX_ENGINE_BUDGET_LEDGER.md`;
- a legal integrated profile envelope.

## 3. Profile Envelopes

| Profile | Integrated fixture | Locked envelope row | Legal measured p95 envelope | Hard engine-owned p95 ceiling |
|---|---|---|---:|---:|
| `interactive-60` | `mixed-pressure-realtime-a` | `BL-MIXED-REALTIME-INT-01` | <= 13.50 ms frame | 16.00 ms |
| `listen-host-60` | `mixed-pressure-realtime-a` | `BL-MIXED-REALTIME-LH-01` | <= 13.50 ms frame | 17.85 ms |
| `headless-20` | `mixed-pressure-headless-a` | `BL-MIXED-HEADLESS-01` | <= 40.00 ms tick | 22.50 ms |

## 4. Mechanical Binding Map — Interactive-60

| Budget class | Gate row | Gate kind | Locked baseline row / absolute gate | Fixture | Capture sheet/result or absolute source | Ledger row |
|---|---|---|---|---|---|---|
| runtime kernel + realtime shell | `engine_runtime orchestration` | regression | `BL-RT-ORCH-01` | `runtime-kernel-a` | `CS-INT-60-CPU-01` / `CR-INT-60-CPU-01` | Interactive runtime row |
| runtime kernel + realtime shell | `engine_runtime_realtime whole-profile progression (interactive)` | regression | `BL-REALTIME-INT-01` | `mixed-pressure-realtime-a` | `CS-INT-60-VK-01` / `CR-INT-60-VK-01` | Interactive runtime row |
| runtime kernel + realtime shell | `engine_runtime_realtime low-latency queue depth (interactive)` | absolute | `ABSOLUTE-GATE` | `mixed-pressure-realtime-a` | absolute budget constitution | Interactive runtime row |
| `L1.5` aggregate | `engine_stream_control activation/prefetch pass (interactive)` | regression | `BL-SVC-STREAM-INT-01` | `streaming-locality-a` | `CS-INT-60-CPU-01` / `CR-INT-60-CPU-01` | Interactive `L1.5` row |
| `L1.5` aggregate | `engine_residency_control residency accounting (interactive)` | regression | `BL-SVC-RESID-INT-01` | `streaming-locality-a` | `CS-INT-60-CPU-01` / `CR-INT-60-CPU-01` | Interactive `L1.5` row |
| `L1.5` aggregate | `engine_memory_control frame reset and pool trim` | regression | `BL-SVC-MEM-01` | `streaming-locality-a` | `CS-CMN-CPU-01` / `CR-CMN-CPU-01` | Interactive `L1.5` row |
| `L1.5` aggregate | `engine_transfer_control decode/staging/upload chain (interactive)` | regression | `BL-SVC-XFER-INT-01` | `streaming-locality-a` | `CS-INT-60-VK-01` / `CR-INT-60-VK-01` | Interactive `L1.5` row |
| `L1.5` aggregate | `engine_runtime queue depth stability` | absolute | `ABSOLUTE-GATE` | `runtime-kernel-a` | absolute budget constitution | Interactive `L1.5` row |
| imaging | `engine_imaging extraction baseline (interactive)` | regression | `BL-IMG-INT-01` | `visibility-realtime-a` | `CS-INT-60-VK-01` / `CR-INT-60-VK-01` | Interactive imaging row |
| imaging | `engine_runtime_realtime mixed-pressure envelope (interactive)` | regression | `BL-MIXED-REALTIME-INT-01` | `mixed-pressure-realtime-a` | `CS-INT-60-VK-01` / `CR-INT-60-VK-01` | Interactive imaging row |
| acoustics | `engine_acoustics solve baseline (interactive)` | regression | `BL-AUDIO-INT-01` | `acoustics-streaming-a` | `CS-INT-60-CPU-01` / `CR-INT-60-CPU-01` | Interactive acoustics row |
| acoustics | `engine_runtime_realtime mixed-pressure envelope (interactive)` | regression | `BL-MIXED-REALTIME-INT-01` | `mixed-pressure-realtime-a` | `CS-INT-60-VK-01` / `CR-INT-60-VK-01` | Interactive acoustics row |
| world snapshot/apply | `engine_world snapshot build` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute budget constitution | Interactive world row |
| world snapshot/apply | `engine_world apply batch integration` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute budget constitution | Interactive world row |
| replay + diagnostics reserve | `deterministic replay checkpoint cadence` | absolute | `ABSOLUTE-GATE` | `replay-determinism-a` | absolute determinism rule | Interactive reserve row |
| replay + diagnostics reserve | `deterministic replay checkpoint compare` | regression | `BL-REPLAY-CMP-01` | `replay-determinism-a` | `CS-CMN-CPU-01` / `CR-CMN-CPU-01` | Interactive reserve row |
| replay + diagnostics reserve | `diagnostics ceiling probe` | absolute | `ABSOLUTE-GATE` | `mixed-pressure-realtime-a` | absolute diagnostics ceiling | Interactive reserve row |
| replay + diagnostics reserve | `degradation-order integrity probe` | absolute | `ABSOLUTE-GATE` | `mixed-pressure-realtime-a` | absolute degradation law | Interactive reserve row |
| mandatory simulation families aggregate | `engine_runtime_realtime mixed-pressure envelope (interactive)` | regression | `BL-MIXED-REALTIME-INT-01` | `mixed-pressure-realtime-a` | `CS-INT-60-VK-01` / `CR-INT-60-VK-01` | Interactive simulation row |

## 5. Mechanical Binding Map — Listen-Host-60

| Budget class | Gate row | Gate kind | Locked baseline row / absolute gate | Fixture | Capture sheet/result or absolute source | Ledger row |
|---|---|---|---|---|---|---|
| runtime kernel + realtime shell | `engine_runtime_realtime whole-profile progression (listen-host)` | regression | `BL-REALTIME-LH-01` | `mixed-pressure-realtime-a` | `CS-LH-60-VK-01` / `CR-LH-60-VK-01` | Listen-host runtime row |
| runtime kernel + realtime shell | `engine_runtime_realtime low-latency queue depth (listen-host)` | absolute | `ABSOLUTE-GATE` | `mixed-pressure-realtime-a` | absolute budget constitution | Listen-host runtime row |
| `L1.5` aggregate | `engine_stream_control activation/prefetch pass (listen-host)` | regression | `BL-SVC-STREAM-LH-01` | `streaming-locality-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host `L1.5` row |
| `L1.5` aggregate | `engine_residency_control residency accounting (listen-host)` | regression | `BL-SVC-RESID-LH-01` | `streaming-locality-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host `L1.5` row |
| `L1.5` aggregate | `engine_memory_control frame reset and pool trim (listen-host)` | regression | `BL-SVC-MEM-LH-01` | `streaming-locality-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host `L1.5` row |
| `L1.5` aggregate | `engine_transfer_control decode/staging/upload chain (listen-host)` | regression | `BL-SVC-XFER-LH-01` | `streaming-locality-a` | `CS-LH-60-VK-01` / `CR-LH-60-VK-01` | Listen-host `L1.5` row |
| `L1.5` aggregate | `engine_runtime queue depth stability` | absolute | `ABSOLUTE-GATE` | `runtime-kernel-a` | absolute budget constitution | Listen-host `L1.5` row |
| `L2.5` aggregate | `engine_net_transport packet encode/decode (listen-host)` | regression | `BL-NET-TX-LH-01` | `network-listen-host-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host `L2.5` row |
| `L2.5` aggregate | `engine_net_sync snapshot/delta publication (listen-host)` | regression | `BL-NET-SYNC-LH-01` | `network-listen-host-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host `L2.5` row |
| `L2.5` aggregate | `engine_net_sync ack backlog and delivery state (listen-host)` | absolute | `ABSOLUTE-GATE` | `network-listen-host-a` | absolute network backlog ceiling | Listen-host `L2.5` row |
| `L2.5` aggregate | `engine_net_latency predict/reconcile/rewind (listen-host)` | regression | `BL-NET-LAT-LH-01` | `network-listen-host-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host `L2.5` row |
| mandatory simulation families | `engine_kinetics batch compute` | regression | `BL-KIN-01` | `destruction-heavy-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host simulation row |
| mandatory simulation families | `engine_field regional solve` | regression | `BL-FIELD-01` | `destruction-heavy-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host simulation row |
| mandatory simulation families | `engine_agents batch cycle` | regression | `BL-AGENT-01` | `agent-cycle-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host simulation row |
| mandatory simulation families | `simulation tier near-footprint envelope (listen-host)` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute tier ceiling | Listen-host simulation row |
| imaging | `engine_imaging extraction baseline (listen-host)` | regression | `BL-IMG-LH-01` | `visibility-realtime-a` | `CS-LH-60-VK-01` / `CR-LH-60-VK-01` | Listen-host imaging row |
| imaging | `engine_runtime_realtime mixed-pressure envelope (listen-host)` | regression | `BL-MIXED-REALTIME-LH-01` | `mixed-pressure-realtime-a` | `CS-LH-60-VK-01` / `CR-LH-60-VK-01` | Listen-host imaging row |
| acoustics | `engine_acoustics solve baseline (listen-host)` | regression | `BL-AUDIO-LH-01` | `acoustics-streaming-a` | `CS-LH-60-CPU-01` / `CR-LH-60-CPU-01` | Listen-host acoustics row |
| acoustics | `engine_runtime_realtime mixed-pressure envelope (listen-host)` | regression | `BL-MIXED-REALTIME-LH-01` | `mixed-pressure-realtime-a` | `CS-LH-60-VK-01` / `CR-LH-60-VK-01` | Listen-host acoustics row |
| world + replay + diagnostics reserve | `engine_world snapshot build` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute budget constitution | Listen-host reserve row |
| world + replay + diagnostics reserve | `engine_world apply batch integration` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute budget constitution | Listen-host reserve row |
| world + replay + diagnostics reserve | `deterministic replay checkpoint cadence` | absolute | `ABSOLUTE-GATE` | `replay-determinism-a` | absolute determinism rule | Listen-host reserve row |
| world + replay + diagnostics reserve | `deterministic replay checkpoint compare` | regression | `BL-REPLAY-CMP-01` | `replay-determinism-a` | `CS-CMN-CPU-01` / `CR-CMN-CPU-01` | Listen-host reserve row |
| world + replay + diagnostics reserve | `diagnostics ceiling probe` | absolute | `ABSOLUTE-GATE` | `mixed-pressure-realtime-a` | absolute diagnostics ceiling | Listen-host reserve row |
| world + replay + diagnostics reserve | `degradation-order integrity probe` | absolute | `ABSOLUTE-GATE` | `mixed-pressure-realtime-a` | absolute degradation law | Listen-host reserve row |

## 6. Mechanical Binding Map — Headless-20

| Budget class | Gate row | Gate kind | Locked baseline row / absolute gate | Fixture | Capture sheet/result or absolute source | Ledger row |
|---|---|---|---|---|---|---|
| runtime kernel + headless shell | `engine_runtime_headless whole-profile progression` | regression | `BL-HEADLESS-01` | `world-apply-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless runtime row |
| runtime kernel + headless shell | `engine_runtime_headless mixed-pressure envelope` | regression | `BL-MIXED-HEADLESS-01` | `mixed-pressure-headless-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless runtime row |
| `L1.5` aggregate | `engine_stream_control activation/prefetch pass (headless)` | regression | `BL-SVC-STREAM-HD-01` | `streaming-locality-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L1.5` row |
| `L1.5` aggregate | `engine_residency_control residency accounting (headless)` | regression | `BL-SVC-RESID-HD-01` | `streaming-locality-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L1.5` row |
| `L1.5` aggregate | `engine_memory_control frame reset and pool trim (headless)` | regression | `BL-SVC-MEM-HD-01` | `streaming-locality-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L1.5` row |
| `L1.5` aggregate | `engine_transfer_control decode/staging/apply chain (headless)` | regression | `BL-SVC-XFER-HD-01` | `streaming-locality-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L1.5` row |
| `L1.5` aggregate | `engine_runtime queue depth stability` | absolute | `ABSOLUTE-GATE` | `runtime-kernel-a` | absolute budget constitution | Headless `L1.5` row |
| `L2.5` aggregate | `engine_net_transport packet encode/decode (headless)` | regression | `BL-NET-TX-HD-01` | `network-headless-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L2.5` row |
| `L2.5` aggregate | `engine_net_sync snapshot/delta publication (headless)` | regression | `BL-NET-SYNC-HD-01` | `network-headless-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L2.5` row |
| `L2.5` aggregate | `engine_net_sync ack backlog and delivery state (headless)` | absolute | `ABSOLUTE-GATE` | `network-headless-a` | absolute network backlog ceiling | Headless `L2.5` row |
| `L2.5` aggregate | `engine_net_latency predict/reconcile/rewind (headless)` | regression | `BL-NET-LAT-HD-01` | `network-headless-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless `L2.5` row |
| mandatory simulation families | `engine_runtime_headless mixed-pressure envelope` | regression | `BL-MIXED-HEADLESS-01` | `mixed-pressure-headless-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless simulation row |
| mandatory simulation families | `simulation tier near-footprint envelope (headless)` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute tier ceiling | Headless simulation row |
| world + replay + storage reserve | `engine_world snapshot build` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute budget constitution | Headless reserve row |
| world + replay + storage reserve | `engine_world apply batch integration` | absolute | `ABSOLUTE-GATE` | `world-apply-a` | absolute budget constitution | Headless reserve row |
| world + replay + storage reserve | `deterministic replay checkpoint cadence` | absolute | `ABSOLUTE-GATE` | `replay-determinism-a` | absolute determinism rule | Headless reserve row |
| world + replay + storage reserve | `deterministic replay checkpoint compare` | regression | `BL-REPLAY-CMP-01` | `replay-determinism-a` | `CS-CMN-CPU-01` / `CR-CMN-CPU-01` | Headless reserve row |
| world + replay + storage reserve | `engine_runtime_headless mixed-pressure envelope` | regression | `BL-MIXED-HEADLESS-01` | `mixed-pressure-headless-a` | `CS-HD-20-CPU-01` / `CR-HD-20-CPU-01` | Headless reserve row |

## 7. Composition Closure Audit

| Profile | Budget classes covered | Gate rows covered | Baseline / absolute closure | Envelope row present | Numeric ledger row present | Seal status |
|---|---:|---:|---|---|---|---|
| `interactive-60` | 7 / 7 | 19 / 19 | same-floor or legal common rows only | yes | yes | mechanically sealed |
| `listen-host-60` | 7 / 7 | 25 / 25 | same-floor or legal common rows only | yes | yes | mechanically sealed |
| `headless-20` | 5 / 5 | 18 / 18 | same-floor, legal common, or absolute rows only | yes | yes | mechanically sealed |

## 8. Rule

Profile legality is mechanically sealed only when every binding row above resolves without profile drift, without missing capture artifacts for regression rows, and without negative headroom in the numeric ledger.

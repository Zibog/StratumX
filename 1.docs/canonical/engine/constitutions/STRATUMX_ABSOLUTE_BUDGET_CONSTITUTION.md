# STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION

## 1. Purpose

This document freezes absolute engine-overhead ceilings for canonical bench profiles.
It governs engine overhead, engine-managed memory, queue growth, bridge growth, storage, decode, and service-layer budgets.
Game-specific content may consume the remaining machine budget, but engine ownership may not exceed the ceilings below.
Numeric constants are authored in `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` and benchmark proof is measured under `STRATUMX_BENCHMARK_PROTOCOL.md`.

## 2. Reference Profiles

| Profile | Role | Canonical Envelope |
|---|---|---|
| `interactive-60` | single-player or client-facing realtime presentation profile | 60 Hz presentation with one local authority and no host-side remote publication burden |
| `listen-host-60` | realtime presentation profile with host-side networking | 60 Hz presentation plus host-side publication, history, and session burden |
| `headless-20` | fixed-step headless profile | 20 Hz authoritative simulation without presentation ownership |

## 3. Canonical Hard Ceilings

### 3.1. Runtime kernel ceilings
- `engine_runtime` orchestration overhead: <= 0.75 ms p95 and <= 1.25 ms p99 per tick on `interactive-60` and `listen-host-60`.
- segmented apply orchestration overhead: <= 1.50 ms p95 and <= 2.50 ms p99 per tick on `interactive-60` and `listen-host-60`.
- queue-barrier overhead inside one tick: <= 0.25 ms p95 and <= 0.50 ms p99.

### 3.2. Lower-stack traversal ceilings
- framework overhead in one critical execution lane: <= 15% of lane wall time at p95 and <= 20% at p99.
- public-handle validation work is illegal on dense execution traversal once a traversal plan is compiled.
- registry participation in compiled traversal is illegal except on cache invalidation or plan rebuild.

### 3.3. Runtime resource service ceilings
- aggregate `L1.5` CPU overhead on `interactive-60`: <= 2.00 ms p95 and <= 3.50 ms p99 per frame.
- aggregate `L1.5` CPU overhead on `listen-host-60`: <= 2.50 ms p95 and <= 4.00 ms p99 per frame.
- transfer upload budget on presentation profiles: <= 32 MiB p95 and <= 64 MiB p99 per frame.
- steady-state read amplification: <= 2.0x requested uncompressed bytes at p95 and <= 3.0x at p99.
- runtime decode/transcode CPU overhead on `interactive-60`: <= 0.50 ms p95 and <= 0.75 ms p99 per frame.
- runtime decode/transcode CPU overhead on `listen-host-60`: <= 0.75 ms p95 and <= 1.00 ms p99 per frame.
- runtime decode/transcode CPU overhead on `headless-20`: <= 1.00 ms p95 and <= 1.50 ms p99 per tick.

### 3.4. Network service ceilings
- aggregate `L2.5` CPU overhead on `interactive-60`: <= 1.00 ms p95 and <= 1.75 ms p99 plus <= 0.05 ms p95 per active remote peer.
- aggregate `L2.5` CPU overhead on `listen-host-60`: <= 1.50 ms p95 and <= 2.25 ms p99 plus <= 0.08 ms p95 per active remote peer.
- per-client sustained gameplay network budget: <= 96 KiB/s p95 with a hard cap of 128 KiB/s.
- correction burst after reconciliation: <= 2 consecutive frames above normal sync cost.

### 3.5. Rewind ceilings
- client-owned rewind window: <= 250 ms.
- authoritative hit-validation rewind window: <= 500 ms.
- rewindable domains are closed by `STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`; all other domains are non-rewindable.

### 3.6. Rendering and visibility ceilings
- scene extraction CPU overhead on `interactive-60`: <= 1.25 ms p95 and <= 2.00 ms p99.
- rendering-side upload staging CPU overhead on `interactive-60`: <= 0.75 ms p95 and <= 1.25 ms p99.
- realtime frame-resource policy CPU overhead on `interactive-60`: <= 0.35 ms p95 and <= 0.50 ms p99.
- total engine-owned GPU frame burden on `interactive-60` and `listen-host-60`: <= 10.00 ms p95 and <= 12.50 ms p99.
- visibility-result staleness on realtime profiles: <= 1 frame.
- material or shader permutation ceiling per canonical family: <= 256 legal runtime-selectable variants, with low-latency profiles targeting <= 64.
- readback on the low-latency presentation path is illegal except explicit diagnostics windows.

### 3.7. Acoustics ceilings
- acoustics-owned runtime buffering, reflection/occlusion, and decode CPU overhead on realtime profiles: <= 0.90 ms p95 and <= 1.30 ms p99.
- acoustics-owned runtime buffering, reflection/occlusion, and decode CPU overhead on `headless-20`: <= 0.40 ms p95 and <= 0.60 ms p99.

### 3.8. Replay and determinism ceilings
- deterministic replay logging overhead: <= 5% wall-time overhead on canonical bench profile.
- replay divergence detection cost: <= 0.25 ms p95 per checkpoint compare.
- checkpoint size, log bytes per tick, save-boundary size, and join-handoff payload size are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` and `STRATUMX_REPLAY_AND_DETERMINISM_CONSTITUTION.md`.

### 3.9. Engine-managed RAM and VRAM ceilings

| Profile | RAM hot set ceiling | RAM warm set ceiling | VRAM hot set ceiling | VRAM transient ceiling |
|---|---:|---:|---:|---:|
| `interactive-60` | 2.50 GiB | 4.50 GiB | 2.00 GiB | 384 MiB |
| `listen-host-60` | 3.00 GiB | 5.50 GiB | 2.00 GiB | 384 MiB |
| `headless-20` | 2.25 GiB | 4.50 GiB | n/a | n/a |

These ceilings apply to engine-owned memory classes and service-owned residency only. Content or mod payloads may not be used to hide engine-side overgrowth.

### 3.10. Storage ceilings
- engine-owned streaming read bandwidth on `interactive-60`: <= 96 MiB/s p95 and <= 128 MiB/s p99.
- engine-owned streaming read bandwidth on `listen-host-60`: <= 112 MiB/s p95 and <= 160 MiB/s p99.
- engine-owned streaming read bandwidth on `headless-20`: <= 128 MiB/s p95 and <= 192 MiB/s p99.
- storage page-open latency on benchmark-floor hardware: <= 1.0 ms p95 for 128 KiB runtime pages.
- page verification CPU cost: <= 0.10 ms p95 per 128 KiB page.

### 3.11. Queue depth ceilings

| Queue | Ceiling |
|---|---:|
| apply queue | <= 16,384 staged records per segment and <= 65,536 staged records aggregate per authoritative tick |
| transfer completion queue | <= 4,096 completion records per frame |
| per-connection publication queue | <= 1,024 publication records and <= 256 KiB queued payload per peer |
| low-latency presentable-frame queue | <= 1 queued presentable frame |

### 3.12. Same-tick bridge ceilings
- same-tick cross-family bridge payload: <= 64 KiB per family-pair per region per tick;
- aggregate same-tick cross-family bridge payload: <= 512 KiB per tick on `interactive-60` and `listen-host-60`, and <= 768 KiB per tick on `headless-20`;
- aggregate same-tick cross-family bridge record count: <= 4,096 records per authoritative tick.


### 3.13. Startup validation and warmup ceilings
- startup manifest validation on benchmark-floor hardware: <= 150 ms p95 and <= 250 ms p99.
- startup pack verification pass before launch: <= 64 MiB verified payload or <= 250 page verifications by default.
- optional pipeline-cache warmup: <= 300 ms p95 and <= 500 ms p99.
- startup may defer additional non-critical verification only as visible maintenance work governed by stream-control law.

### 3.14. Engine-owned CPU composability ceilings
- aggregate engine-owned CPU overhead excluding game-specific family work on `interactive-60`: <= 7.00 ms p95 and <= 9.50 ms p99 per frame.
- aggregate engine-owned CPU overhead excluding game-specific family work on `listen-host-60`: <= 8.00 ms p95 and <= 10.50 ms p99 per frame.
- aggregate engine-owned CPU overhead excluding game-specific family work on `headless-20`: <= 14.00 ms p95 and <= 20.00 ms p99 per tick.
- local crate ceilings must compose beneath the aggregate ceilings above; a locally legal budget that makes the aggregate ceiling impossible is illegal.

## 4. Hard Law

- any engine change that crosses one hard ceiling is a blocker-grade regression;
- ceilings may move only by explicit canon revision;
- profile-specific optimizations may improve a ceiling but may not create hidden side channels that break replay, legality, or ownership law.

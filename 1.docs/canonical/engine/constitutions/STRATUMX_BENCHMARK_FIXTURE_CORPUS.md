# STRATUMX_BENCHMARK_FIXTURE_CORPUS

## 1. Purpose

This document freezes the canonical benchmark fixtures and baseline corpus used by StratumX performance validation.
Performance gates are illegal without a named fixture from this corpus.

## 2. Fixture Set

| Fixture ID | Scope | Canonical envelope |
|---|---|---|
| `lower-stack-traversal-a` | lower substrate | 5,000,000 entities, 8 hot component signatures, 3 compiled traversal-plan classes |
| `runtime-kernel-a` | runtime kernel | empty tick, minimal tick, orchestration, contention, queue-depth, and service-window probes on canonical benchmark floor |
| `world-apply-a` | world/apply | 256 apply segments, 8 family tags, 65,536 staged records aggregate |
| `streaming-locality-a` | stream/residency/transfer | 1,024 page requests, mixed immediate/predicted/maintenance classes, 20% cancellation churn |
| `network-listen-host-a` | transport/sync/latency | 8 remote peers, 384 near actors, 96 KiB/s p95 per peer target, active ack/reconcile burden |
| `network-headless-a` | transport/sync/latency | 16 remote peers, 512 authority-owned actors, 96 KiB/s p95 per peer target, replay/history/storage pressure, and no presentation path |
| `visibility-realtime-a` | imaging/transfer | 9 active near regions, 250,000 instances, 2,048 instance clusters, 128 submission buckets max |
| `mixed-pressure-realtime-a` | integrated realtime mixed pressure | near destruction, streaming churn, visibility churn, listen-host sync burden, replay/checkpoint pressure, and startup-warmed runtime packs on the benchmark floor |
| `mixed-pressure-headless-a` | integrated headless mixed pressure | authority-only world progression, network burden, replay/checkpoint pressure, storage verification pressure, and no presentation path on the benchmark floor |
| `replay-determinism-a` | replay/checkpoint | 100-tick interactive cadence with mixed bridge traffic and save/join handoff checkpoints |
| `destruction-heavy-a` | kinetics/field/world | 65,536 destructible cells in active near footprint with bounded cross-chunk spill |
| `agent-cycle-a` | agents | 1,024 active near actors, 8 squads, bounded perception cache, schedule pressure, and far-agent event churn |
| `service-normalization-a` | inference/generation | 10,000 normalized requests or outputs with typed payload envelopes and no world writes |
| `content-transform-a` | content/resource pipeline | 512 runtime pages transformed from pre-cooked products with deterministic pack metadata |
| `startup-assembly-a` | startup | manifest validation, profile selection, runtime wiring, and warm cache assembly on canonical benchmark floor |
| `acoustics-streaming-a` | acoustics/streaming | 96 active voices, 2 decode workers, 512 streaming audio pages |

## 3. Baseline Corpus Rule

- every performance gate row must bind to one or more fixture ids from the table above;
- locked baselines are keyed by fixture id, capture profile, hardware floor id, backend id, benchmark protocol id, and canon stack version;
- changing fixture data without changing the fixture id is illegal;
- adding a new gate row without a fixture id is illegal.

## 4. Binding

The canonical benchmark corpus is interpreted against `STRATUMX_HARDWARE_PROFILE_CONTRACT.md`, `STRATUMX_PROFILE_COMPOSITION_PROOF.md`, and `STRATUMX_BENCHMARK_PROTOCOL.md`.


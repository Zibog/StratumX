# STRATUMX_PERFORMANCE_CONSTITUTION

## 1. Purpose

This document defines the canonical performance law of StratumX.

## 2. Performance Principles

- critical execution lanes are sacred;
- execution law must remain branch-light and allocation-light;
- lower stack must stay cache-conscious, descriptor-driven, and traversal-plan-driven;
- critical simulation families must scale under runtime-owned parallelism;
- runtime resource services must prevent stalls, thrash, and allocator turbulence from poisoning critical progression law;
- network runtime services must preserve authority and responsiveness without hidden queue explosions;
- service layers may be expensive, but may not poison critical progression law;
- performance regressions are blocker-grade events, not advisory notes.

## 3. Canonical Metric Classes

### 3.1. Correctness-linked metrics
Determinism stability, apply legality success, snapshot consistency integrity, sync consistency integrity.

### 3.2. Latency metrics
Tick time, phase time, family step time, service request latency, input-to-apply latency, reconcile latency, upload latency.

### 3.3. Throughput metrics
Entities per second, regions per second, outputs per second, requests per second, packets per second, chunks per second.

### 3.4. Allocation metrics
Allocation count per critical operation, burst allocations, retained memory after cycle, transient arena churn.

### 3.5. Memory metrics
Snapshot size, delta size, world growth, family scratch size, resident set size, hot-set pressure, transfer backing pressure.

### 3.6. Contention metrics
Lock frequency, lock duration, blocked-worker ratio, queue pressure, fence wait ratio.

### 3.7. Scaling metrics
1→N thread efficiency, region scaling, batch scaling, synthesis scaling, sync scaling, stream-scaling.

### 3.8. Stability metrics
Output variance, phase jitter, frame pacing variance, runtime overhead drift, degradation trigger rate, correction burst rate.

## 4. Reproducibility Attachments

- `STRATUMX_BENCHMARK_PROTOCOL.md`
- `STRATUMX_HARDWARE_PROFILE_CONTRACT.md`
- `STRATUMX_BENCHMARK_FIXTURE_CORPUS.md`
- `STRATUMX_LOCKED_BASELINE_TABLE.md`
- `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`
- `STRATUMX_DIAGNOSTICS_CEILING_LAW.md`

## 5. Performance Law Attachments

The following constitutional attachments are mandatory and authoritative:

- `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`
- `STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md`
- `STRATUMX_SIMULATION_TIER_CONSTITUTION.md`
- `STRATUMX_CROSS_FAMILY_SOLVE_ORDER.md`
- `STRATUMX_STREAMING_AND_PACK_LOCALITY_LAW.md`
- `STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`
- `STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`
- `STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md`
- `STRATUMX_REPLAY_AND_DETERMINISM_CONSTITUTION.md`
- `STRATUMX_PERFORMANCE_GATE_MATRIX.md`
- `STRATUMX_CRATE_PERFORMANCE_BUDGETS.md`

## 6. Hard Performance Laws

- no avoidable heap churn in steady-state critical execution lanes;
- no broad authoritative-state locks across compute phases;
- no resource-service work may silently dominate mandatory progression law;
- no network-service work may silently dominate mandatory progression law;
- no service-layer work may silently dominate mandatory progression law;
- no family may redefine global scheduling for local convenience;
- no performance waiver is legal without an explicit canon update;
- no profile may omit absolute budget proof for runtime, streaming, transfer, sync, rewind, rendering upload, diagnostics, or hardware-floor law;
- no benchmark gate is legal without a named canonical fixture, a fixed hardware-floor id, a benchmark protocol id, and either a locked baseline id or an explicit absolute gate.

## 7. Canonical Target Direction

- target zero avoidable heap allocations in steady-state critical execution lanes;
- target near-zero lock contention in legal runtime-owned parallel paths;
- target deterministic fixed-step progression overhead so small that whole-engine cost is dominated by real family work, not orchestration;
- target shallow queue depth in low-latency interactive profiles;
- target aggressive scaling under legal parallel workloads;
- target brutal regression sensitivity so performance drift is caught early.

## Additional Binding

Performance legality is derived from absolute ceilings, per-crate planning envelopes, benchmark protocol, backend-keyed locked baselines, and the integrated profile-composition proof.

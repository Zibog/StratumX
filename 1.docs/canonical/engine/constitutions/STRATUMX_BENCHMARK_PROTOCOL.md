# STRATUMX_BENCHMARK_PROTOCOL

## 1. Purpose

This document freezes the measurement protocol for canonical StratumX performance proof.
Locked baselines and regression gates are illegal unless measured with this protocol or a stricter canon revision that explicitly supersedes it.

## 2. Build and Runtime Posture

- release build only;
- canonical benchmark runs use the same stack marker as `STACK_VERSION`;
- diagnostics, readback probes, and debug-only tracing must be disabled unless the gate explicitly measures diagnostics law;
- the benchmark machine must run no concurrent workload that sustains > 2% CPU or > 5% storage bandwidth during the run;
- CPU affinity must be pinned to the physical-core set declared by the hardware floor contract;
- memory must be NUMA-local when NUMA exists;
- OS power mode must be fixed to a performance mode;
- turbo or boost posture must be fixed for the entire run: either disabled or held to a documented all-core benchmark posture; switching posture between baseline capture and regression capture is illegal;
- GPU rows must fix backend identity, driver family, driver posture, and feature tier for the entire run.

## 3. Measurement Method

### 3.1. Warmup
- minimum warmup: 3 seconds or 500 iterations, whichever is longer;
- warmup samples are never included in locked baseline values.

### 3.2. Sampling
- minimum timed samples per run: 2,000 for microbench rows and 300 for integrated fixture rows;
- minimum repeated runs per benchmark row: 5;
- the canonical reported value is the median-of-runs p95 for latency metrics and median-of-runs steady-state mean for throughput metrics;
- if coefficient of variation exceeds 3% on the timed sample set, the run is unstable and invalid for baseline capture.

### 3.3. Storage and GPU rules
- storage gates must be measured against the canonical runtime-page size from the numeric source of truth;
- GPU-side timing must use timestamp-query or equivalent backend timing when the backend exposes it; otherwise the gate is illegal for locked baseline capture;
- wall-clock only is legal for startup and CPU-only rows, not for GPU frame-time baselines.

## 4. Benchmark Sheet Fields

Every locked baseline capture must produce a registered capture-sheet artifact and record:
- stack marker;
- hardware floor id;
- benchmark fixture id;
- capture profile;
- benchmark protocol id `BENCH-PROTO-v2`;
- capture sheet id from `STRATUMX_BASELINE_CAPTURE_SHEETS.md` and capture-result id from `STRATUMX_BASELINE_CAPTURE_RESULTS.md`;
- compiler version and target triple;
- backend identity for GPU-facing rows or `CPU-ONLY` for CPU-only rows;
- driver family and driver posture for GPU-facing rows or `n/a` for CPU-only rows;
- OS power mode;
- turbo or boost posture;
- baseline metric values.

## 5. Rule

A locked baseline, regression comparison, or gold-package performance claim is illegal unless it can be reproduced from the hardware floor contract, fixture corpus, profile-composition proof, engine budget ledger, baseline-capture sheets, baseline-capture results, numeric source of truth, locked baseline table, and this protocol.

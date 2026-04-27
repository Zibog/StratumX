# STRATUMX_EXECUTION_CONSTITUTION

## 1. Purpose

This document defines the canonical execution law of StratumX.

It fixes how the engine progresses in time, how runtime profiles execute work, how world access is opened and closed, how staged changes are applied, and how one world instance remains under one active execution authority.

## 2. Scope

This document covers:

- fixed-step execution;
- tick and frame distinction;
- execution units;
- phase order;
- read / compute / stage / apply / egress cycle;
- world access windows;
- runtime profile differences;
- runtime resource service coordination;
- network service coordination;
- authority model per world instance;
- execution outputs.

## 3. Execution Principles

### 3.1. One execution constitution
All runtime profiles follow one execution law.

### 3.2. One active runtime authority per world instance
A world instance has one active runtime authority.

### 3.3. Runtime owns execution
`engine_runtime` owns execution law, scheduling law, phase law, apply law, frame pacing law, and global parallel ownership.

### 3.4. World owns truth
`engine_world` owns authoritative world truth.

### 3.5. Read before compute, compute before apply
No legal execution path skips this order.

### 3.6. Runtime resource services are governed, not sovereign
Resource services run only inside runtime-owned legality windows.

### 3.7. Network runtime services are governed, not sovereign
Transport, sync, and latency control run only inside runtime-owned legality windows and never replace authoritative world ownership.

## 4. Time Model

### 4.1. Fixed simulation step
Simulation progression is fixed-step by law.

### 4.2. Tick
A tick is the canonical simulation progression unit.

### 4.3. Frame
A frame is a profile-facing presentation cadence. A frame is not the canonical authority over world progression.

### 4.4. Profile relation
Headless and realtime profiles may expose different frame behavior, but they do not change tick law.

## 5. Execution Units

- tick
- optional substep
- phase
- job
- batch
- staged output
- apply unit
- stream activation unit
- sync publication unit

## 6. Canonical Phase Order

```text
ingress
read
compute
resource
authority-sync
stage
apply
egress
diagnostics
```

## 7. Phase Definitions

### 7.1. Ingress
Commands, actions, and legal external requests enter runtime.

### 7.2. Read
Runtime opens legal read windows over authoritative state and legal snapshots.

### 7.3. Compute
Critical simulation families and legal service layers compute over legal inputs.

### 7.4. Resource
Runtime resource services may activate, stage, upload, reclaim, or report pressure under runtime law.

### 7.5. Authority-sync
Network runtime services may build snapshots, deltas, predictions, corrections, and rewind validations under runtime law.

### 7.6. Stage
Families and service layers emit staged outputs. No family becomes world authority here.

### 7.7. Apply
Runtime orders and applies staged changes into `engine_world`.

### 7.8. Egress
Events, outputs, and external-facing execution products leave the runtime step.

### 7.9. Diagnostics
Runtime records execution diagnostics and timing products.

## 8. World Access Windows

### 8.1. Read window
A legal read-only or effectively immutable access window over authoritative or snapshot state.

### 8.2. Stage window
A legal window where families emit staged outputs without direct authoritative mutation.

### 8.3. Apply window
The only canonical window where authoritative world truth changes.

## 9. Runtime Profiles

### 9.1. `engine_runtime`
Defines execution constitution and shared runtime law.

### 9.2. `engine_runtime_headless`
Uses the same constitution in simulation-first mode.

### 9.3. `engine_runtime_realtime`
Uses the same constitution in realtime interactive mode.

## 10. Runtime Resource Services

The canonical runtime resource services are:

- `engine_stream_control`
- `engine_residency_control`
- `engine_memory_control`
- `engine_transfer_control`

They are runtime-governed service owners for streaming, residency, memory, and transfer concerns. They do not redefine execution law.

## 11. Network Runtime Services

The canonical network runtime services are:

- `engine_net_transport`
- `engine_net_sync`
- `engine_net_latency`

They are runtime-governed service owners for online transport, sync, and latency control. They do not redefine execution law.

## 12. Service Layers

The canonical service layers are:

- `L3.0. Model Systems`
- `L3.1. Synthesis Systems`
- `L3.2. Resource Systems`

They are runtime-governed consumers and producers inside legal execution windows. They do not redefine execution law.

## 13. Runtime Outputs

A legal runtime step emits:

- progressed authoritative world state;
- staged outputs integrated by apply;
- events;
- diagnostics;
- execution status.

## 14. Failure and Degradation Boundaries

- critical simulation progression may not be skipped by auxiliary pressure;
- runtime resource services may change priority, cadence, or budgets under canonical policy, but may not silently redefine authority or bypass runtime law;
- network runtime services may thin bandwidth, publish rate, or interest scope under canonical policy, but may not silently redefine authority or bypass runtime law;
- service layers may be deferred or degraded under canonical degradation law;
- runtime authority may not be duplicated as a fallback strategy.

## 15. Canonical Laws

- one world instance;
- one active runtime authority;
- one execution constitution;
- read before compute;
- compute before apply;
- apply changes world truth;
- resource and network services follow runtime law.

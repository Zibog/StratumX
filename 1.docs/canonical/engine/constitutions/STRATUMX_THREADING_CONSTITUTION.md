# STRATUMX_THREADING_CONSTITUTION

## 1. Purpose

This document defines the canonical threading law of StratumX.

## 2. Global Threading Ownership

Global parallel ownership belongs to `engine_runtime`.

No lower substrate crate, simulation family, resource service, network service, service layer, or startup layer owns the global threading constitution.

## 3. Threading Principles

### 3.1. Runtime-owned parallel policy
Threading policy is centralized in runtime.

### 3.2. Parallel-ready layers
Lower substrate, simulation families, runtime resource services, network runtime services, and service layers are parallel-ready, not parallel-sovereign.

### 3.3. Shared-read, staged-write
Parallel reads are legal under runtime execution law. Authoritative writes are staged then applied.

### 3.4. No nested global schedulers
No family or service layer defines a competing global execution constitution.

## 4. Canonical Parallel Forms

### 4.1. Region-based parallelism
Used for region, chunk, tile, and field-oriented work.

### 4.2. Island-based parallelism
Used for local contact and dynamics clusters.

### 4.3. Agent-batch parallelism
Used for agent and society simulation.

### 4.4. Pipeline parallelism
Used for content/resource processing, decode/upload chains, and staged processing chains.

### 4.5. Service-layer parallelism
Used for model tasks, synthesis workloads, network packing, and resource workloads inside runtime-owned legality windows.

## 5. Barrier Model

Barriers are canonical between:

- read and apply;
- independent compute phases where legal ordering matters;
- critical simulation and service-layer work where state visibility must be preserved;
- packet publication and authoritative correction where per-connection order must be preserved.

## 6. Lock Policy

Canonical lock policy is:

- immutable/shared descriptor reads first;
- lock-minimal runtime coordination;
- no broad authoritative-world locks across whole compute phases;
- no family-owned scheduler locks;
- ordered publication only where connection, stream, or apply law requires it.

## 7. Threading by Layer

- `L-1..L0` = parallel-ready substrate, no global ownership
- `L0.5` = immutable/read-heavy shared property substrate
- `L1` = global parallel ownership
- `L1.5` = runtime-governed resource service parallelism
- `L2` = runtime-governed critical simulation parallelism
- `L2.5` = runtime-governed network service parallelism
- `L3.0..L3.2` = runtime-governed service-layer parallelism
- `L4` = startup orchestration, not a competing global scheduler

## 8. Canonical Laws

- one runtime-owned global thread policy;
- many parallel-ready consumers;
- no nested execution sovereignties;
- no direct authoritative world writes from uncontrolled parallel paths.

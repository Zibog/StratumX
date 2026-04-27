# STRATUMX_IMPLEMENTATION_HANDOFF

## 1. Purpose

This document defines the non-negotiable implementation handoff rules for the engine canonical package.

## 2. Handoff Reading Set

Implementation starts only from the following authoritative set:
- root canon;
- constitutional laws;
- package closure docs;
- the entire implementation package;
- level and crate documents.

No implementation-facing document may be treated as optional once handoff begins.

## 3. Non-Negotiable Rules

### 3.1. No upward dependency from `L1.5` into `L3.x`
`engine_stream_control` and `engine_transfer_control` may not depend directly on `engine_content`.

### 3.2. `engine_content` relation is product-level, not crate-level
`engine_content` may emit prepared resource products and runtime-pack products.
Those products become legal `L1.5` inputs only after `engine_startup` selects, validates, and binds them, or when equivalent externally prepared inputs are supplied.

### 3.3. Runtime phase order is fixed
Implementation must preserve:
- ingress;
- read;
- compute;
- resource;
- authority-sync;
- stage;
- apply;
- egress;
- diagnostics.

### 3.4. Critical simulation families are runtime- and memory-aware
`engine_kinetics`, `engine_field`, and `engine_agents` must preserve the canonical dependency relation to `engine_runtime` and `engine_memory_control`.

### 3.5. Implementation order is dependency-legal
Implementation must preserve the canonical topological order defined in `STRATUMX_IMPLEMENTATION_ORDER.md`, including authoritative intra-phase order.

### 3.6. API-law to skeleton legality is non-negotiable
Implementation-facing crate contract skeletons must remain legal under `constitutions/STRATUMX_API_CONTRACT_LAW.md`.

### 3.7. Critical execution lane law is non-negotiable
Traversal plans, access descriptors, stable-handle boundaries, and dense execution handles must obey `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md`.

### 3.8. Simulation tiers, solve order, and queue ceilings are non-negotiable
Implementation must preserve `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`, `constitutions/STRATUMX_CROSS_FAMILY_SOLVE_ORDER.md`, and the queue ceilings frozen by `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`.

### 3.9. Absolute budgets and profile memory envelopes are non-negotiable
Runtime, streaming, transfer, sync, rewind, rendering, replay, RAM, VRAM, and allocator classes must stay within `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md` and `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`.

### 3.10. Implementation starter docs are normative
`STRATUMX_FIRST_COMPILEABLE_SKELETON.md` and `STRATUMX_FIRST_HEADLESS_RUNTIME_MILESTONE.md` are part of the authoritative implementation package and may not contradict root or constitutional law.

### 3.11. Package closure docs remain live
Any change that breaks `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, or `16_AUDIT_READINESS_MATRIX.md` breaks sealed handoff status.

## 4. Sealed Handoff Rule

The engine package is sealed for implementation handoff only when:
- `14_ACCEPTANCE_MATRIX.md` is fully pass;
- `16_AUDIT_READINESS_MATRIX.md` has no open blockers;
- no implementation document contradicts root or constitutional law.

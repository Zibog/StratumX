# STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW

## 1. Purpose

This document freezes how latency-sensitive traversal and access are represented and constrained inside canonical crates.

## 2. Canonical Terms

- `critical execution lane` = a latency-sensitive, allocation-sensitive, cache-sensitive path inside one canonical crate.
- `traversal plan` = compiled, cacheable execution descriptor for one legal query signature.
- `dense execution handle` = internal dense token used inside one compiled traversal plan.
- `stable public handle` = externally visible stable token that may cross crate boundaries.

## 3. Laws

### 3.1. Handle split law
- stable public handles are legal at boundaries;
- dense execution handles are legal only inside compiled traversal and batch execution;
- no canonical crate may expose dense execution handles as public API.

### 3.2. Traversal plan law
A canonical traversal plan is frozen by:
- component set;
- access mode;
- locality class;
- partitionability;
- invalidation rule.

### 3.3. Query cache law
- traversal plans must be cached by query signature;
- cache invalidation is legal only on registry-shape change, layout-class change, or access-legality change;
- per-iteration rebuild of traversal plans is illegal.

### 3.4. Registry participation law
- registry is structural truth only;
- registry lookups are illegal inside the steady-state body of a compiled traversal lane;
- registry may participate only at plan build, invalidation, and diagnostics boundaries.

### 3.5. Access descriptor law
Read/write legality must be machine-readable in:
- access descriptor;
- traversal plan;
- runtime scheduling legality.

### 3.6. Allocation law
- no general allocator traffic is legal in steady-state critical execution lanes;
- transient scratch is legal only through owning allocator classes frozen by `STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`.

## 4. Lower-Stack Ownership Map

- `engine_handle` owns stable-public validation law;
- `engine_storage_layout` owns layout classes;
- `engine_storage_access` owns read/write windows and traversal entry law;
- `engine_ecs_registry` owns structural shape truth;
- `engine_ecs_query` owns traversal plans, query signatures, and invalidation law.

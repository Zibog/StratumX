# Threading Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Status of This Document

This document is the root threading summary map.

The authoritative normative law is:

- `constitutions/STRATUMX_THREADING_CONSTITUTION.md`

## Global Ownership

Global parallel execution ownership belongs only to:

- `engine_runtime`

No other crate owns a competing global execution constitution.

## Parallel-Ready Lower Layers

The lower substrate and world stack are designed to support:

- parallel-safe reads;
- deterministic access;
- partition-friendly traversal;
- staged mutation;
- controlled apply;
- explicit critical execution lane discipline.

## Runtime Resource Services

`L1.5` services are parallel-ready but not parallel-sovereign.
They may batch IO, decode, upload, residency, and allocator work, but they do so only under runtime-owned legality windows, queue ceilings, and budget ceilings.

## Parallel-Ready Upper Families

Critical simulation families, network services, and service layers are all parallel-ready, but their execution is runtime-governed.

## Canonical Rule

- one world instance;
- one active runtime authority;
- one runtime-owned global thread policy;
- many parallel-ready families and service layers;
- no competing global schedulers;
- no hidden queue ownership outside the canon;
- no hidden allocator ownership inside critical execution lanes.

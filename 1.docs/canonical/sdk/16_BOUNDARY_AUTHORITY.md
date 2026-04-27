# 16 BOUNDARY AUTHORITY

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This law defines what `sdk/L5` may publish across the engine-to-tools boundary and what it may never own.

## Boundary owner set
`L5` is authoritative only for:
- typed ingress packets;
- ingress control rows;
- typed egress observations and metrics;
- compatibility facts and legality verdicts;
- opaque handles, refs, cursors, epochs, and artifact refs;
- immutable bridge snapshots and ordered bridge batches.

## Explicit non-owners
`L5` is never authoritative for:
- editor panel/view/layout/selection state;
- prefab instances, local overrides, or nested prefab policy;
- data-layer trees, streaming-region policy, or world-partition authoring;
- validation rules, bake queues, build graphs, release packaging policy;
- assistant session state, planner goals, campaign orchestration, or product workflow state.

## Crossing law
Anything crossing the `L4 -> L5 -> L6+` boundary must be one of:
- a typed packet or observation;
- a metric row or verdict row;
- a handle/ref/cursor/epoch/artifact reference;
- an immutable snapshot or batch projection.
If a proposed crossing requires mutable authoring truth, UI semantics, or orchestration semantics, it is illegal at `L5`.

## Audit checks
- every `L5` field class resolves to one of the owner-set classes above;
- every upper-stack need is satisfied through refs, cursors, snapshots, or verdicts rather than leaked authority;
- no editor or tooling concept is named as if `L5` owned it.

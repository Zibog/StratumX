# L5 Internal Contract Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Physical planes
- immutable bridge snapshots;
- ordered ingress lanes;
- immutable egress batches and cursors.

## Write contract
- reads immutable fact snapshots;
- resolves verdicts and legality through compiled lookup tables;
- applies transport policy without semantic widening;
- publishes packets or controls through ordered ingress lanes.

## Read contract
- receives typed `L4` publications through public batch or snapshot surfaces;
- normalizes them without semantic widening;
- exposes fanout-safe immutable views upward as snapshots, batches, and cursors.

## Internal law
No internal contract may require:
- editor panel state;
- entity/component authoring schema;
- prefab instance schema;
- data-layer trees or streaming-cell policy;
- assistant session state;
- campaign or planner state.

No internal contract may duplicate engine truth into a second mutable authority store.
Any upper-layer richness must be composed from bridge publications by `L6`, never pre-owned by `L5`.

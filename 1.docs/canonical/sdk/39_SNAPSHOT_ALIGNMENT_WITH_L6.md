# Snapshot Alignment With L6

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical alignment
- `L5` immutable bridge snapshots feed `L6 snapshot_plane` consumers;
- `L5` immutable egress batches feed `L6 stream_plane` and diagnostics consumers;
- `L5` ordered ingress lanes accept `L6 command_envelopes` lowering;
- `L5` artifact refs feed `L6 artifact_plane`, `preview_runtime`, `build_runtime`, and `release_runtime` readers without transferring ownership.

## Alignment rules
`L6` may derive:
- hierarchy projections;
- content/dependency indices;
- prefab and layer legality checks;
- runtime bridge projections;
- build and release manifests.

`L5` may not pre-own any of the above on behalf of `L6`.

## Forbidden
- `L6` bypassing `L5` to reach engine internals;
- `L5` reshaping itself into editor authority state;
- duplicate mutable truth stores spanning both layers;
- storing panel-ready projections inside bridge snapshots.

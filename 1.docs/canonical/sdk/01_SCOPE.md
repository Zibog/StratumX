# Scope

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This package freezes the `L5` SDK bridge as the only legal typed connection between engine `L4` and the upper tools stack.

It defines:
- write-side bridge envelopes;
- read-side bridge envelopes;
- compatibility, transport, and legality surfaces;
- opaque handle and ref classes;
- artifact-ref publication toward tools;
- replay-safe batches, snapshots, cursors, and epochs.

It does not define:
- engine internals;
- editor authority state;
- prefab, data-layer, package, or plugin models;
- assistant runtime truth;
- studio orchestration truth;
- planner truth;
- build/release ownership.

## Editor-dream support posture
For the target editor, `L5` must support very large authoring workflows by exposing:
- compact stable identity carriers;
- explicit version and epoch fields;
- cursor-safe observation/metric publication;
- immutable fact snapshots suitable for `L6` index and validation runtimes;
- opaque artifact refs usable by preview/build/release without leaking artifact ownership.

Everything richer than the above belongs in `tooling/` or `editor/`, not in `sdk/`.

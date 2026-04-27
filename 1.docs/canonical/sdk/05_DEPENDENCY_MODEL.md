# Dependency Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Allowed direction
- engine `L4` publishes through `L5`;
- upper tools consume engine exports through `L5`;
- legality and compatibility evaluation may depend on exported facts only;
- `L5` may align immutable bridge snapshots to `L6 snapshot_plane`, `L6 stream_plane`, and `L6 artifact_plane` expectations without inheriting any `L6` ownership.

## Forbidden direction
- `L5 -> L6` authority inversion;
- `L5 -> tooling` ownership of validation, preview, build, release, workspace coordination, or package orchestration;
- `L5 -> editor` panel, layout, selection, inspector, suite, or plugin semantics;
- `L5 -> assistant runtime` ownership;
- `L5 -> studio orchestration` ownership;
- `L5 -> planner` ownership;
- `L5 -> engine` deep internals beyond public `L4` bundles.

## Dependency law
`L5` is a dependency funnel, not a dependency magnet.
If a desired feature requires any of the following, it is already too high for `L5` and must move upward:
- mutable authoring truth;
- prefab override resolution;
- data-layer legality;
- validation rule execution;
- preview composition;
- build queue ownership;
- release packaging policy;
- package/dependency resolution;
- product UI state, layout, focus routing, or inspector staging.

## Editor-dream reading
`L5` may carry ids, refs, handles, facts, verdicts, epochs, and artifact refs required by the dream editor.
`L5` may not define what an outliner row, inspector field, prefab override panel, or data-layer browser means.

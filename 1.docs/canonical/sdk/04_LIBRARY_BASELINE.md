# Library Baseline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Pinned External Baseline

Required across L5 root and level-local surfaces:
- `serde`
- `thiserror`

Conditionally allowed only when a root law or local 10_LIBRARIES files under each `levels/*/` layer package document explicitly justifies them:
- `smallvec`
- `crossbeam`
- `tracing`
- `bitflags`
- `indexmap`
- `semver`

## Forbidden External Classes

- hidden databases
- hidden disk caches
- hidden model runtimes
- auto-growing graph stores
- uncontrolled GPU ownership
- reflection-heavy dynamic registries in hot publication paths
- editor UI libraries
- direct engine-internal libraries beyond declared public L4 export surfaces
- direct tooling libraries

## Root Freeze Rule

No SDK root or level-local document may introduce an external dependency outside the pinned or conditionally allowed set above without first updating this root baseline.

L5 libraries must move typed data, not invent new authority, shadow state, or hidden stores.

## Version

SX-CANON/1.0.24/STACK-v30 sdk package.

# Library Baseline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Pinned External Baseline

Required across tooling root and level-local surfaces:
- `serde`
- `thiserror`

Conditionally allowed only when a root law or local 10_LIBRARIES files under each tooling level or family pack document explicitly justifies them:
- `smallvec`
- `tracing`
- `crossbeam`

## Forbidden External Classes

- hidden authority stores
- hidden databases
- unbounded queues
- unbounded caches on hot or warm paths
- uncontrolled GPU residency
- silent disk growth
- editor mutation bypasses around `L6`
- direct engine-internal libraries outside sanctioned `L5` intake surfaces

## Root Freeze Rule

No tooling root or level-local document may introduce an external dependency outside the pinned or conditionally allowed set above without first updating this root baseline.

Tooling libraries may coordinate, derive, validate, preview, build, release, plan, or report. They may not become a second truth owner.

## Version

SX-CANON/1.0.24/STACK-v30 tooling package.

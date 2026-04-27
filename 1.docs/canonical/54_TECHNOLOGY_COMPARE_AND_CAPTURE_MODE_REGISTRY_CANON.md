# Technology Compare And Capture Mode Registry Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This registry freezes the canonical ids for compare and capture behavior.

| Family | Capture mode ids | Compare mode ids |
|---|---|---|
| diagnostics/trace | `capture.trace.bundle`, `capture.trace.failure_slice` | `compare.trace.triplet`, `compare.trace.topology` |
| resource/floor | `capture.floor.bundle`, `capture.pressure.timeline` | `compare.floor.recovery`, `compare.pressure.threshold` |
| certification | `capture.cert.bundle`, `capture.cert.failed_run` | `compare.cert.scenario`, `compare.cert.freeze_review` |
| heavy simulation | `capture.domain.window`, `capture.domain.restore_triplet` | `compare.domain.replay`, `compare.domain.baseline_triplet` |
| render/audio | `capture.frame.slice`, `capture.audio.route` | `compare.frame.regression`, `compare.audio.profile` |
| material-first terrain/world | `capture.material.bundle`, `capture.terrain.bundle`, `capture.world.day_zero_bundle` | `compare.material.response_triplet`, `compare.terrain.layer_triplet`, `compare.world.day_zero_triplet` |
| material-centric proof | `capture.material.coverage_bundle`, `capture.material.branch_bundle`, `capture.material.freeze_bundle` | `compare.material.branch_triplet`, `compare.material.trigger_triplet`, `compare.material.freeze_triplet` |

## Law
- compare and capture ids are canonical signal ids and may not be renamed inside sdk or tooling payloads;
- any route that emits a capture or compare result must reference one id from this registry;
- compare without baseline-bearing mode id is forbidden where baseline law applies;
- material-owned preview, validation, and freeze routes may not invent branch-local compare/capture ids outside the `capture.material.*` and `compare.material.*` families.

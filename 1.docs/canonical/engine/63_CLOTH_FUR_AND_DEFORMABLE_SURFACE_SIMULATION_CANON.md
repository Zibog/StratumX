# Cloth Fur And Deformable Surface Simulation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for soft-surface solve, wind response, wetness response, contact legality, deformable recovery, and the coupling line to coverage systems.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `SoftSurfaceStripSet` | cloth panels, fur patches, and deformable surface ids |
| `SolverRungState` | active solver rung, convergence band, and downgrade ceiling |
| `WetResponseField` | wetness-driven damping and mass gain coefficients |
| `ContactImpulseTape` | contact impulses, anchor constraints, and violation markers |
| `ShapeRecoveryAnchor` | last-good pose/solver anchor for recovery |
| `SoftReplayWindow` | stable replay horizon for response comparison |
| `CoverageCouplingState` | declared coupling from soft solve into coverage representation tiers |

## Exact state machine
`relaxed -> solving -> wind_loaded / wet_loaded -> contact_loaded -> diverged / recovered -> certified`

## Exact phase order
1. ingest wind and wetness bands
2. solve constraints
3. resolve contacts
4. publish divergence markers
5. emit recovery anchor and compare slice
6. publish coverage-coupling state when fur/hair richness is affected

## Exact coupling boundaries
- may read wind and wetness bands only via declared fields
- may not own animation intent or skeletal truth
- must expose one solver rung downgrade path before failure publication
- must treat promoted local strand-like richness as optional coupling from engine `106`, not as baseline soft-solve law

## Exact fail / denial families
- `soft.solver.diverge`
- `soft.contact.band_illegal`
- `soft.wet.coefficient_missing`
- `soft.anchor.invalid`

## Exact resource envelope
cpu `<= 3.4 ms`; gpu overlays `<= 1.0 ms`; ram `<= 320 MiB`; disk/IO none on hot path

## Exact replay window
stable 6-second response replay with rung snapshots at contact, divergence, and recovery

## Exact certification duties
- retain solver rung digest, wet-response capture, contact violation board
- retain baseline/failed/recovery compare bundle
- retain divergence timestamp and anchor id
- retain any representation-ladder handoff to engine `106` when present

## Exact legal recovery actions
- drop one solver rung
- restore shape-recovery anchor
- rerun response compare before certifying

## Exact domain command families
- `verify.soft_response`
- `compare.solver_triplet`
- `capture.divergence_frame`
- `recover.soft_anchor`

## Current posture
`document_gold / doc_closed_impl_open`

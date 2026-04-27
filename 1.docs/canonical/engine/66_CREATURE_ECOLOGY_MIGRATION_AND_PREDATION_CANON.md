# Creature Ecology Migration And Predation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for species population bands, habitat saturation, migration corridors, predation pressure, and avoidance recovery.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `SpeciesPopulationBandSet` | species counts, age buckets, and mobility class |
| `HabitatSaturationLedger` | capacity, scarcity, and overpressure state |
| `MigrationCorridorGraph` | corridor edges, block state, and legality tier |
| `PredationPressureField` | hunt pressure, prey risk, and threat avoidance |
| `AvoidancePolicySet` | avoidance routes, hazard bands, and fallback tiers |
| `EcologyRecoveryAnchor` | last-good corridor/pressure baseline |

## Exact state machine
`stable -> migrating -> pressured -> predating / avoiding -> blocked -> recovered -> certified`

## Exact phase order
1. advance population buckets
2. score habitat saturation
3. resolve migration edges
4. apply predation/avoidance
5. publish corridor blockers and recovery anchor

## Exact coupling boundaries
- may read fire, hydrology, and society hazard bands only as pressure inputs
- may not mutate wound or tactics truth directly
- must publish one corridor legality tier for every route compare

## Exact fail / denial families
- `ecology.route.edge_break`
- `ecology.predation.pressure_illegal`
- `ecology.habitat.capacity_missing`
- `ecology.anchor.invalid`

## Exact resource envelope
cpu `<= 3.8 ms`; gpu overlays `<= 0.6 ms`; ram `<= 384 MiB`; disk/IO none on hot path

## Exact replay window
stable 18-second migration replay with edge snapshots and pressure checkpoints

## Exact certification duties
- retain corridor graph snapshot, habitat ledger digest, predation pressure board, recovery anchor bundle
- retain migration compare bundle
- retain first blocker code

## Exact legal recovery actions
- lower distant corridor tier
- restore ecology anchor
- rerun migration compare before certifying

## Exact domain command families
- `verify.ecology_route`
- `compare.migration_triplet`
- `capture.predation_pressure`
- `recover.ecology_anchor`



## Exact editor entrypoints
- `btn.eco.author_species_profile`
- `btn.eco.bind_migration_corridor`
- `btn.eco.inspect_hazard_chain`
- `btn.eco.simulate_migration_window`
- `btn.eco.compare_triplet`
- `btn.eco.capture_evidence`
- `btn.eco.recover_baseline`
- `btn.eco.certify_pack`

## Phase-3 proof slices
- hunger, weather, and hazard pressure reroute migration legally;
- predation pressure does not bypass corridor legality;
- every ecology reroute exports one reason fragment to `engine/76`.

## Required publications
- `event.eco.migration_window_applied.v1` with `species_scope`, `corridor_ref`, `hazard_band`, `trace_ref`, `artifact_ref`;
- `artifact.eco.route.triplet` with baseline/failed/recovery and predation pressure digest.


## Current posture
`document_gold / doc_closed_impl_open`

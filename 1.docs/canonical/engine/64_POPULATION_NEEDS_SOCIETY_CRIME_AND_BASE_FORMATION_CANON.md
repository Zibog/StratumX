# Population Needs Society Crime And Base Formation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for population scheduling, need pressure, social legality, crime pressure, faction posture, and base formation.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `PopulationActorLedger` | actor id, schedule slot, need vector, faction affinity |
| `NeedPressureField` | aggregated need pressure buckets by district and time band |
| `CrimePressureField` | crime propensity, unresolved triggers, and enforcement posture |
| `FactionZoneState` | faction control, hostility posture, and zone legality |
| `BaseFormationNodeSet` | base ownership, support radius, and fallback nodes |
| `SocietyRecoveryAnchor` | last-good legality anchor for schedule/faction state |

## Exact state machine
`scheduled -> pressured -> conflicting -> criminalized / stabilized -> recovered -> certified`

## Exact phase order
1. advance schedules
2. integrate needs
3. resolve crime pressure
4. resolve faction/base legality
5. publish blockers and compare slices

## Exact coupling boundaries
- may consume hydrology, weather, and tactics fields as read-only pressure inputs
- may not mutate tactics or wound truth directly
- must expose one legality anchor for every compare-bearing publication

## Exact fail / denial families
- `society.schedule.conflict_unresolved`
- `society.crime.band_missing`
- `society.faction.zone_illegal`
- `society.base.anchor_missing`

## Exact resource envelope
cpu `<= 4.8 ms`; gpu overlays `<= 0.7 ms`; ram `<= 512 MiB`; disk/IO none on hot path

## Exact replay window
stable 24-second schedule replay with legality checkpoints per time band

## Exact certification duties
- retain legality board, crime bucket digest, faction/base anchor bundle
- retain compare manifest for schedule, crime, and faction posture
- retain first blocker and next legal action

## Exact legal recovery actions
- restore previous legality anchor
- drop one pressure tier
- rerun schedule compare and faction legality board

## Exact domain command families
- `verify.society_schedule`
- `compare.population_triplet`
- `capture.crime_pressure_board`
- `recover.society_anchor`



## Exact editor entrypoints
- `btn.soc.author_need_curve`
- `btn.soc.bind_faction_rule`
- `btn.soc.inspect_reason_chain`
- `btn.soc.simulate_population_step`
- `btn.soc.compare_triplet`
- `btn.soc.capture_evidence`
- `btn.soc.recover_baseline`
- `btn.soc.certify_pack`

## Phase-3 proof slices
- scarcity and lack of supply increase need pressure until crime posture changes;
- faction and base formation change only through canonical legality rows;
- every population step exports one reason fragment to `engine/76` and one retained compare anchor.

## Required publications
- `event.soc.population_step_applied.v1` with `actor_scope`, `need_band`, `crime_delta`, `trace_ref`, `artifact_ref`;
- `artifact.soc.reason.triplet` with baseline/failed/recovery digests and faction/base legality board.


## Current posture
`document_gold / doc_closed_impl_open`

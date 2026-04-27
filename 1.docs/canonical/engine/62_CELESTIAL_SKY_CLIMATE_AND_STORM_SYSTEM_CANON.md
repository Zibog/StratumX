# Celestial Sky Climate And Storm System Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own time-of-day sky truth, climate bands, storm fronts, wind fields, precipitation bands, visibility pressure, and lawful publication for phase-2 material-world proof.

## Exact truth objects
- `celestial_time_state`
- `climate_band_state`
- `storm_front_state`
- `wind_vector_field`
- `precipitation_band`
- `visibility_humidity_state`
- `storm_recovery_anchor`

## Exact state machine
`time_tick -> climate_integrate -> front_seed -> front_advect -> precipitate -> visibility_shift -> dissipate / persist -> published`

## Exact phase order
1. advance celestial and seasonal clocks.
2. integrate climate and humidity bands.
3. advect fronts and wind vectors.
4. solve precipitation, wetness drivers, and visibility implications.
5. publish weather, sky, and coupling slices.
6. seal compare digest and recovery anchor.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | engine `60` large-world region bands; engine `91` volumetric media capacities; engine `98` budget arbitration | never owns render sky composition or audio mix |
| publishes | engine `49` fire wind/wetness drivers; engine `61` evaporation drivers; editor-visible storm track evidence | may not bypass the degrade ladder declared by `69` old-hardware floor |

## Exact compare / capture / certification law
| Action family | Allowed ids | Pack |
|---|---|---|
| compare | `compare.storm.front_track` | `pack.storm_long_range_visibility` |
| capture | `capture.storm.visibility_bundle` | `pack.storm_long_range_visibility` |
| recover | `recover.storm.*`, `recover.sky.*` | `pack.storm_long_range_visibility` |
| certify | phase-2 certification through editor `67` only | `pack.storm_long_range_visibility` |

## Phase-2 brutal proof slice
`far storm front visible at distance -> wind and precipitation bands arrive -> local visibility pressure changes -> compare digest and recovery remain lawful`

## Failure and denial families
- `storm.front.seed_missing`
- `storm.wind.field_break`
- `storm.visibility.band_break`
- `storm.recovery.anchor_invalid`

## Resource envelope
- CPU: green <= 1.0 ms, yellow <= 1.6 ms, orange <= 2.4 ms, red > 2.4 ms.
- GPU: truth publication itself CPU-owned; any simulation-driven sky media request orange > 1.2 ms, red > 1.8 ms.
- RAM: green <= 224 MiB band history, orange > 320 MiB, red > 416 MiB.
- disk / IO: climate snapshots only on declared checkpoints; red on any blocking write > 8 ms.

## Ordered degrade ladder
- reduce distant turbulence detail;
- reduce cloud self-shadow refresh request rate;
- reduce non-critical media compare density;
- retain front identity, wind direction legality, and precipitation-band causality.

## Current posture
`document_gold / doc_closed_impl_open`

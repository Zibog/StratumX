# Fire Wetness Wind Thermal Propagation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the runtime truth contract for ignition, burn propagation, wetness suppression, wind transport, smoke-bearing thermal spread, and certification-safe recovery.

## Exact truth objects
| Truth object | Role | Required publication |
|---|---|---|
| `ThermalCellLedger` | authoritative thermal energy, ignition age, extinguish state | thermal band + denial lineage |
| `FuelMoistureField` | per-material wetness and burn suppression coefficients | ignition blocker digest |
| `WindBucketField` | quantized wind direction, lift class, and transport tier | advection tier + ladder state |
| `IgnitionFrontSet` | active flame fronts and pending propagation edges | front lineage and retry state |
| `SmokeDensityBandField` | volumetric smoke bands linked to thermal cells | smoke advection bands |
| `HeatRecoveryAnchor` | last-good baseline for coupled fire/weather verification | recovery anchor id |

## Exact state machine
`idle -> primed -> igniting -> propagating -> wet_suppressed / rain_suppressed / budget_suppressed -> smoldering -> extinguished -> recovered`

## Exact phase order
1. ingest moisture, wind, and precipitation bands.
2. resolve ignition legality per material response row.
3. propagate fronts and drying influence.
4. publish smoke/thermal bands and suppression reason.
5. seal compare digest and recovery anchor.

## Exact coupling boundaries
- may read material ignition coefficients from engine `50` and weather bands from engine `62`.
- may not mutate hydrology truth directly; wetness is consumed as a declared input.
- must publish first blocking code when wetness, rain, or budget suppresses propagation.
- must expose whether drying resumed ignition or ignition remained lawfully denied.

## Exact compare / capture / certification law
| Action family | Allowed ids | Pack |
|---|---|---|
| compare | `compare.fire_weather.coupling` | `pack.fire_weather_smoke` |
| capture | `capture.fire_weather.band_bundle` | `pack.fire_weather_smoke` |
| recover | `recover.fire_weather.*` | `pack.fire_weather_smoke` |
| certify | phase-2 certification through editor `67` only | `pack.fire_weather_smoke` |

## Phase-2 brutal proof slices
- wet grass refuses ignition.
- sustained heat dries the target and ignition resumes lawfully.
- wind changes spread direction without losing cause lineage.
- rain suppresses flame without erasing heat history.
- smoke advection follows wind buckets and remains comparable after recovery.

## Exact fail / denial families
- `fire.thermal.coupling_break`
- `fire.fuel.wetness_lock`
- `fire.front.edge_missing`
- `fire.rain.suppression_desync`
- `fire.smoke.advection_gap`

## Exact resource envelope
cpu `<= 3.2 ms`; gpu diagnostics `<= 0.8 ms`; ram `<= 384 MiB`; disk/IO none on hot path

## Current posture
`document_gold / doc_closed_impl_open`

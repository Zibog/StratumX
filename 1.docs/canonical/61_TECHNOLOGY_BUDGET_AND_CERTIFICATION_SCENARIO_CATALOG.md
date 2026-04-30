# 61_TECHNOLOGY_BUDGET_AND_CERTIFICATION_SCENARIO_CATALOG

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Name the canonical scenario packs that tie mixed heavy domains to budgets, compare modes, and baseline families.

## Canonical scenario rows

| Scenario id | Canonical pack id | Mixed families | Floor relevance | Compare mode | Baseline family |
|---|---|---|---|---|---|
| `scenario.hydrology.persistence` | `pack.hydrology_persistence` | hydrology + persistence | yes | `cmp.triplet.mass_continuity` | `baseline.hydrology.mass_ledger` |
| `scenario.fire.weather.smoke` | `pack.fire_weather_smoke` | fire + weather + smoke | yes | `cmp.triplet.thermal_spread` | `baseline.fire_weather.chain` |
| `scenario.fur.cloth.weather` | `pack.fur_cloth_weather` | cloth + fur + wetness + wind | yes | `cmp.triplet.soft_surface_contact` | `baseline.soft_surface.floor` |
| `scenario.population.tactics.damage` | `pack.population_tactics_damage` | population + tactics + wounds | yes | `cmp.triplet.reason_causality` | `baseline.population_reason` |
| `scenario.combined.old_floor` | `pack.combined_old_hardware_floor` | world-scale mixed pack | yes | `cmp.triplet.old_floor_mixed` | `baseline.old_floor.mixed` |

## Dream-scene benchmark scenarios
| Scenario id | Canonical pack id | Mixed families | Floor relevance | Compare mode | Baseline family |
|---|---|---|---|---|---|
| `scenario.old_pc.photoreal_tunnel` | `pack.photoreal_old_hardware` | lighting + shadows + volumetrics + transient muzzle flash | yes | `compare.photoreal.old_floor_triplet` | `baseline.photoreal.old_floor` |
| `scenario.distant_storm_arrival` | `pack.climate_front_storm_theater` | climate + visibility + far-world causality | yes | `compare.climate.front_arrival_triplet` | `baseline.climate.front_arrival` |
| `scenario.mass_destruction_urban_block` | `pack.destruction_topology_collapse` | topology + collapse + fragment aftermath | yes | `compare.destruction.block_triplet` | `baseline.destruction.block` |
| `scenario.first_playable_proof_region` | `pack.first_playable_proof_region` | terrain + sky + tunnel + audio + destruction + capture | yes | `compare.first_playable.proof_lane` | `baseline.first_playable.proof_lane` |

## Law
Aliases are forbidden.
Every certification scenario reference must use the canonical pack ids above and point at one retained baseline family.

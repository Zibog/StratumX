# 65_HEAVY_DOMAIN_RESOURCE_ENVELOPE_AND_CERTIFICATION_PACK_CANON

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Declare coarse resource envelopes for mixed heavy-domain certification packs.

| Pack id | CPU threshold | GPU threshold | RAM ceiling | Disk/IO ceiling | Degrade order | Last-good baseline |
|---|---:|---:|---:|---:|---|---|
| `pack.hydrology_persistence` | 7.0 ms | 2.0 ms | 900 MB | 40 MB/s | sample density -> update radius -> distant debug off | `baseline.hydrology.mass_ledger` |
| `pack.fire_weather_smoke` | 8.5 ms | 4.0 ms | 1100 MB | 55 MB/s | smoke detail -> ember count -> rain suppression detail | `baseline.fire_weather.chain` |
| `pack.fur_cloth_weather` | 6.0 ms | 3.0 ms | 800 MB | 20 MB/s | strand density -> solver iterations -> distant contact traces | `baseline.soft_surface.floor` |
| `pack.population_tactics_damage` | 9.5 ms | 2.5 ms | 1200 MB | 25 MB/s | agent reasoning depth -> tactic lookahead -> wound detail | `baseline.population_reason` |
| `pack.combined_old_hardware_floor` | 12.0 ms | 8.0 ms | 1500 MB | 65 MB/s | mixed ladder ordered by `69` | `baseline.old_floor.mixed` |

## Certification law
Every certification pack must declare:
- compare mode;
- evidence append rule;
- freeze blocker family;
- first legal recovery action;
- first downgraded feature if the pack includes visual certification.

# First-Layer To Frame Workload Atlas Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This atlas is the coarse technology crosswalk from scene phenomena to first-layer workload families.
It is not the fine operator manifest; that belongs to `66`, `72`, and `76`.

## Atlas rows
| Phenomenon | First workload families | Active canonical pack | Primary proof surface |
|---|---|---|---|
| rainfall fills container, leaks, dries, restores | hydrology, persistence, compare | `pack.hydrology_persistence` | editor `121`, tooling `90` |
| far storm wall with readable horizon and lightning | celestial-weather, visibility, degrade | `pack.storm_long_range_visibility` | editor `136`, `135` |
| cloth and fur under wind, contact, wetness | soft-surface, weather, LOD pressure | `pack.fur_cloth_weather` | editor `125` |
| firefight with cover break, wounds, morale shift | tactics, wounds, destruction, population | `pack.population_tactics_damage` | editor `137`, `138` |
| migration reroutes under weather and hunger | ecology, weather, hazard, route compare | `pack.ecology_migration` | editor `134`, `137` |
| dialogue shifts faction/world state | semantic, population, persistence | `pack.semantic_consequence_chain` | editor `137` |
| tunnel flash shadow smoke burst | fire-light-media, visibility, performance | `pack.tunnel_flash_shadow_media` | editor `135` |
| mixed old-hardware traversal and combat | all major heavy families | `pack.combined_old_hardware_floor` | editor `126`, tooling `91` |

## Atlas law
Every workload row must point to one pack id, one primary proof surface, and one compare mode family.

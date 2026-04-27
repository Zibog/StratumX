# World Authoring And Runtime Validation Route Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze one world-family route where authoring, chunk rebuild, validation, simulation, save/restore, compare, capture, and degraded posture are read as a single system.
World is not a thin container. It is the owned package for terrain, chunk topology, materials, environment, placements, persistence, and validation identity.

## Canonical world-family route
`open/create world -> establish world identity -> author terrain/material/environment/placements -> rebuild invalidated chunks -> validate world legality -> simulate/inspect -> save/restore -> compare/capture -> certify or deny`

## World-family documents owned by this route
- `world/50_WORLD_PACKAGE_FORMAT_CANON.md`
- `world/51_WORLD_IDENTITY_AND_NAMESPACE_CANON.md`
- `world/52_WORLD_CHUNK_INVALIDATION_AND_REBUILD_CANON.md`
- `world/53_WORLD_STREAMING_PROFILE_AND_RESIDENCY_CANON.md`
- `world/54_WORLD_MATERIAL_REGISTRY_LINKAGE_CANON.md`
- `world/55_WORLD_PERSISTENCE_AND_SAVE_RESTORE_CANON.md`
- `world/56_WORLD_VALIDATION_AND_DEGRADED_POSTURE_CANON.md`

## Exact route classes
| World class | Author controls | Validation / inspect controls | Simulate controls | Save / restore | Compare / capture | Recovery / deny |
|---|---|---|---|---|---|---|
| world identity and namespace | `btn.world.open_world`, `btn.world.save_world` | world identity and namespace inspect surfaces | n/a | world manifest save/restore | evidence rows when freeze-relevant | deny on namespace drift or identity ambiguity |
| terrain / chunk topology / rebuild | `btn.terrain.import_heightmap`, `btn.terrain.sculpt_primary`, `btn.terrain.smooth_patch`, `btn.terrain.flatten_patch`, `btn.terrain.rebuild_chunks`, `btn.terrain.load_chunks`, `btn.terrain.save_chunks` | invalidation and rebuild inspectors | owning workload preview rows | chunk save/load routes | compare/capture rows for chunk baseline | deny on unresolved invalidation or stale rebuild |
| material layers / biome / aftermath | `btn.terrain.paint_layer`, `btn.terrain.paint_biome_mask`, `btn.material.*` | layer-weight, material-linkage, overlay inspectors | material consequence preview rows | save-safe material linkage persistence | compare/capture on world/material baselines | deny on unresolved layer or linkage truth |
| sky / weather / environment | `btn.sky.bind_sky_profile`, `btn.sky.set_time_of_day`, `btn.sky.set_weather_regime`, `btn.sky.bind_cloud_profile` | environment binding inspect rows | weather/storm simulation rows | environment state persistence | compare/capture environment baselines | deny on illegal or incomplete environment bindings |
| placements and content state | editor `107` / `108` rows | placement legality and runtime binding inspect rows | owning domain simulate rows | placement persistence routes | compare/capture rows | deny on placement identity or runtime binding drift |
| world validation and degraded posture | `btn.world.validate_world` | validation drift, chunk/posture inspect rows | preview current degraded posture | retain save/restore-safe baselines | capture validation bundle | deny on unresolved legality, persistence, or residency blockers |

## Root law for world closure
A world route is not closed unless the operator can answer all of the following without leaving the declared contour:
- which world package is authority;
- which chunks are invalid, rebuilt, stale, loaded, or saved;
- which material registry and local instances are linked into the world;
- which streaming/residency posture is currently lawful;
- whether the world can be restored safely;
- which degraded posture, if any, is active.

## Runtime validation law
World validation is not just content linting.
It must prove:
- world identity continuity;
- chunk rebuild closure;
- material/environment linkage closure;
- streaming profile legality;
- save/restore continuity;
- first degraded posture if old-floor or constrained runtime conditions apply.

## Forbidden shortcuts
- No terrain save may persist unresolved chunk invalidation.
- No world package may bind environment or material state through hidden editor-local state.
- No validation pass may report green while namespace, linkage, residency, or restore blockers remain unresolved.

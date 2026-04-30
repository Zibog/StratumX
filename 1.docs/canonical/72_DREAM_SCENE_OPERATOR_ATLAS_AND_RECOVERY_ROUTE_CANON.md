# Dream Scene Operator Atlas And Recovery Route Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Map each dream-scene operator class to one lawful primary route and one lawful recovery route.

## Recovery atlas
| Workload class | Primary controls | Primary route family | Engine owners | Certification packs | First legal recovery action |
|---|---|---|---|---|---|
| material / terrain / structure | phase-2 rows in `editor/110` | `route.material.*`, `route.terrain.*`, `route.struct.*` | `engine/49`, `engine/50` | `pack.material_surface_response`, `pack.terrain_crater_truth`, `pack.structure_cascade_aftermath` | owning recover row in `editor/110` |
| hydrology / fire / storm | phase-2 rows in `editor/110` | `route.hyd.*`, `route.fire.*`, `route.storm.*` | `engine/49`, `engine/61`, `engine/62`, `engine/73` | `pack.hydrology_persistence`, `pack.fire_weather_smoke`, `pack.storm_long_range_visibility` | owning recover row or `btn.timeline.recover_restore_anchor` |
| society / tactics / ecology / wound / semantic / nav / inventory / quest / reason | phase-3 rows in `editor/110` | owning route families | `engine/64–76` | phase-3 packs registered in `81` | owning recover row in `editor/110` |
| presentation/runtime | phase-4 rows in `editor/110` | render/audio/anim/ui/vfx route families | `engine/86`, `engine/88–96` | phase-4 packs registered in `81` | owning recover row or `btn.recover.drop_one_rung` |
| mixed old-hardware floor | `btn.worldfloor.simulate_mixed_pack`, `btn.hardware.simulate_floor_pack` | `route.worldfloor.*`, `route.hardware.*` | `engine/69`, `engine/98`, `engine/102` | `pack.combined_old_hardware_floor`, `pack.large_world_old_floor` | `btn.worldfloor.recover_last_good`, `btn.hardware.recover_last_good` |
| brutal proof-region relay | `btn.project.bootstrap`, `btn.sim.play_slice`, `btn.build.package`, `btn.launch.verify_first_result`, freeze rows | `route.project.*`, `route.sim.*`, `route.build.*`, `route.export.*`, `route.launch.*`, `route.freeze.*` | multi-owner by retained bundle and active proof-region recipe | `pack.brutal_proof_region_relay` plus active mixed packs | `btn.freeze.recover_freeze_posture`, then owning domain recover rows if the blocker is local |

## Law
- recovery may not jump to a different proof-region recipe or a different retained baseline family;
- a mixed-pack blocker may not be resolved by pretending the active pack disappeared;
- relay recovery must preserve build/export/launch lineage instead of restarting from blank state.

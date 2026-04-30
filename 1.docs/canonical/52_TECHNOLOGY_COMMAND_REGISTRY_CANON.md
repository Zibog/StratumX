# Technology Command Registry Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This registry is the authoritative command-id law for canonical operator actions.
Labels may vary by editor surface; command ids may not.

| Command class | Canonical command ids | Required route context |
|---|---|---|
| new | `new.project`, `new.world`, `new.material_profile`, `new.surface_family` | must allocate one stable identity and one authoring scope |
| open | `open.project`, `open.world_package`, `open.bundle`, `open.baseline` | must resolve one authority-owned source path or ref |
| save | `save.project`, `save.world_package`, `save.material_profile`, `save.terrain_chunks` | must name the authority owner and target identity |
| save_as | `save_as.project`, `save_as.material_profile`, `save_as.surface_family` | must create one new identity and preserve source lineage |
| import | `import.heightmap`, `import.texture_set`, `import.material_seed`, `import.mesh_payload` | must resolve canonicalization profile and source lineage |
| author | `author.profile`, `author.rule`, `author.binding`, `author.threshold`, `author.material_archetype`, `author.response_profile` | must resolve one mutating tooling route |
| bind | `bind.surface_family`, `bind.response_profile`, `bind.material_instance_stack`, `bind.biome_overlay` | must resolve one mutating tooling route and one stable target ref |
| inspect | `inspect.slice`, `inspect.overlay`, `inspect.reason_chain`, `inspect.blocker`, `inspect.material_stack`, `inspect.surface_family` | must resolve one read-only route |
| paint | `paint.terrain.layer`, `paint.terrain.biome_mask`, `paint.terrain.aftermath_overlay` | must name brush scope, target layer, and dirty region |
| sculpt | `sculpt.terrain.height`, `sculpt.terrain.smooth`, `sculpt.terrain.noise_stamp` | must name brush scope, target patch, and rebuild policy |
| flatten | `flatten.terrain.patch`, `flatten.terrain.ribbon` | must name target region and reference plane |
| rebuild | `rebuild.terrain.chunks`, `rebuild.material_cache`, `rebuild.binding_graph` | must name invalidation scope and post-rebuild focus |
| simulate | `simulate.local`, `simulate.window`, `simulate.floor_mix`, `simulate.material_response`, `simulate.sky_weather` | must resolve one preview/sim route and one focus target |
| compare | `compare.baseline`, `compare.triplet`, `compare.freeze_review`, `compare.material_triplet`, `compare.terrain_triplet` | must name compare mode and baseline family |
| capture | `capture.snapshot`, `capture.timeline_triplet`, `capture.cert_bundle`, `capture.material_bundle`, `capture.reason_bundle` | must resolve artifact class and retention class |
| recover | `recover.rollback_anchor`, `recover.last_good`, `recover.failed_run`, `recover.material_baseline`, `recover.terrain_baseline` | must name rollback anchor or failed-run reference |
| certify | `certify.pack`, `certify.scenario`, `certify.freeze_review`, `certify.material_pack`, `certify.terrain_pack` | must resolve pack id and evidence posture |


## Material-first command note
The active registry reserves the following day-zero commands as non-optional command classes:
- `author.material_archetype`;
- `bind.surface_family`;
- `bind.response_profile`;
- `mutate.layer_weight`;
- `mutate.biome_overlay`;
- `simulate.material_response`;
- `rebuild.terrain.chunks`;
- `save.terrain_chunks`;
- `open.terrain_chunks`.

## Command law
- every button, shortcut, palette entry, or scripted operator action must lower to one canonical command id from this registry;
- aliases are allowed only at label level, never at id level;
- a command without one declared route family is forbidden;
- terrain and material day-zero actions may not live as untyped UI verbs.


## Base-shell and advanced-authoring command additions
| Command class | Canonical command ids | Required route context |
|---|---|---|
| validate | `validate.world` | must resolve validation scope, blocker publication, and retained verdict artifact |
| focus | `focus.shell_surface`, `focus.restore_previous_surface` | must preserve or legally reset selection and publish shell-state change |
| smooth | `smooth.terrain.patch` | must name patch scope, dirty region, and rebuild implication |
| duplicate | `duplicate.material_profile` | must create lineage-preserving identity and publish pending-save posture |
| bind | `bind.material.microdetail_profile`, `bind.material.weather_modulation`, `bind.sky.profile`, `bind.sky.cloud_profile`, `bind.audio.zone_profile_world_surface`, `bind.audio.priority_ducking_policy` | must resolve target truth owner and pending-save posture |
| set | `set.sky.time_of_day`, `set.sky.weather_regime` | must declare parameter legality and downstream environment publication |
| preview | `preview.material.burn`, `preview.audio.audibility_free_camera`, `preview.audio.obstruction_vs_occlusion`, `preview.audio.indoor_outdoor_transition` | must name preview scope, artifact retention, and denial path |
| assign | `assign.audio.emitter_class_world_source` | must resolve world placement linkage and binding legality |
| inspect | `inspect.audio.voice_subtitle_legality` | must resolve legality scope and retained verdict posture |

## Material-centric command additions
Add canonical verbs:
- `bind.material_visual_response`
- `bind.material_acoustic_profile`
- `bind.material_light_response`
- `set.material_cheap_runtime_rung`
- `preview.material_light_response`
- `preview.material_acoustic_response`
- `inspect.material_coverage`
- `validate.material_route_closure`

## Workspace, windowing, extension, and assistant command additions
| Command class | Canonical command ids | Required route context |
|---|---|---|
| open | `open.workspace_stage_world`, `open.workspace_stage_simulation`, `open.workspace_stage_capture`, `open.extension_manager`, `open.assistant_dock` | must preserve legal focus anchor and publish active workspace/stage |
| load/save/reset | `load.window_layout`, `save.window_layout`, `reset.window_layout`, `restore.default_layout` | must resolve one layout identity, one shell scope, and one recovery rule |
| detach/split | `detach.panel`, `split.viewport.vertical`, `split.viewport.horizontal` | must preserve one lawful viewport owner and publish throttling posture when multi-view is active |
| toggle | `toggle.extension_mount` | must resolve one extension bundle id, one mount scope, and one capability grant |
| install | `install.extension_bundle` | must resolve source lineage, signature/trust verdict, and mount-disabled default posture |
| apply/revert | `apply.assistant_proposal`, `revert.assistant_apply` | must retain proposal lineage, affected routes, rollback anchor, and evidence chain |


## Dream-stack deep command additions
| Command class | Canonical command ids | Required route context |
|---|---|---|
| assign | `assign.terrain_material_set`, `assign.structural_class` | must resolve one stable target ref and one validated schema family |
| bind | `bind.structural_support_group`, `bind.load_path_profile`, `bind.leak_geometry`, `bind.weather_front_seed`, `bind.body_topology_class`, `bind.groom_representation_rung` | must resolve exact truth owner and saved-artifact posture |
| inject | `inject.weather_front`, `inject.rain_window`, `inject.fire_seed`, `inject.tactical_sandbox_scenario` | must resolve deterministic seed or authored input family |
| replay | `replay.destruction`, `replay.weather_evolution`, `replay.fire_spread`, `replay.hydrology_restore`, `replay.tactical_chain`, `replay.wound_chain`, `replay.ballistic_chain` | must resolve retained replay artifact and compare mode |
| certify | `certify.old_hardware_photoreal_pack`, `certify.distant_storm_pack`, `certify.mass_destruction_pack`, `certify.hydrology_persistence_pack`, `certify.fur_wind_pack` | must resolve one canonical pack id and retained evidence posture |
| open | `open.lab_destruction`, `open.lab_hydrology`, `open.lab_climate_theater`, `open.lab_society_tactics`, `open.lab_wound_species`, `open.lab_fur_certification`, `open.lab_photoreal_old_floor`, `open.playbook_first_proof` | must preserve legal focus anchor and project/world scope |

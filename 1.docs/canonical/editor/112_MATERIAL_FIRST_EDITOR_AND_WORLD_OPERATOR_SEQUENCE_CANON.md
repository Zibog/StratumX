> Phase 4 reconciliation note
>
> `editor/112` describes operator order, not a second manifest.
> Every exact `btn.*` id referenced here must already exist in `editor/110`.
> Material-proof closure is now resolved through `btn.material.capture_proof_artifacts` and `btn.material.review_freeze_blockers` in the authoritative manifest.

# Material First Editor And World Operator Sequence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the editor's base operator sequence for building a believable world from material law first, not from mesh trivia first.

## Core principle
The editor must let an operator decide what a thing is before deciding how pretty it looks.
The canonical sequence is therefore:
1. choose or create project;
2. choose or create world;
3. import heightmap;
4. sculpt, smooth, and flatten terrain;
5. paint surface layers and biome overlays;
6. assign material archetypes and response profiles;
7. bind texture, microdetail, weather, and damage/aftermath layers;
8. bind sky, time of day, weather regime, and cloud profile;
9. inspect in free camera;
10. preview consequences;
11. save chunks and world;
12. validate world;
13. compare, capture, recover only when needed;
14. freeze, build, and export later.

## Day-zero world flow
`btn.project.new_project/open_project -> btn.world.open_world_package -> btn.terrain.import_heightmap -> btn.terrain.sculpt_primary/smooth_patch/flatten_patch -> btn.terrain.paint_layer/paint_biome_mask -> btn.material.new_profile/duplicate_profile/assign_archetype/bind_surface_family/bind_response_profile -> btn.material.bind_texture_stack/bind_microdetail_profile/bind_weather_modulation -> btn.sky.bind_sky_profile/set_time_of_day/set_weather_regime/bind_cloud_profile -> btn.view.viewport/outliner/inspector/content_browser -> btn.material.preview_bullet_hit/preview_blast/preview_wetness/preview_burn -> btn.terrain.save_chunks + btn.world.save_world_package + btn.world.validate_world`

## Material-first law
- the editor may not begin from a model and only later ask what it is made of;
- every terrain layer and every authorable object must resolve one archetype, one surface family, and one response profile before simulation claims correctness;
- texture sets are presentation layers, not behavior owners;
- aftermath belongs to world/material truth, not to temporary decals alone.

## Sequence-to-manifest law
This document may not invent exact ids that are absent from `editor/110`.
Every promoted button below is freeze-blocking if missing from the manifest.

## Operator sequence by surface

| Stage | Owning surface | Mandatory exact buttons | Visibility class |
|---|---|---|---|
| project and world shell | editor `82` / `108` | `btn.project.new_project`, `btn.project.open_project`, `btn.world.open_world_package`, `btn.world.save_world_package`, `btn.world.validate_world` | base-shell mandatory |
| canonical import | editor `107` + `108` | `btn.import.heightmap_source`, `btn.terrain.import_heightmap` | base-shell mandatory |
| terrain shape | editor `108` | `btn.terrain.sculpt_primary`, `btn.terrain.smooth_patch`, `btn.terrain.flatten_patch` | base-shell mandatory |
| terrain meaning | editor `108` + editor `92` | `btn.terrain.paint_layer`, `btn.terrain.paint_biome_mask`, `btn.material.assign_archetype`, `btn.material.bind_surface_family`, `btn.material.bind_response_profile` | base-shell mandatory |
| material presentation | editor `92` | `btn.material.new_profile`, `btn.material.duplicate_profile`, `btn.material.bind_texture_stack`, `btn.material.bind_microdetail_profile`, `btn.material.bind_weather_modulation`, `btn.material.save_profile_as` | base-shell mandatory |
| sky context | editor `94` | `btn.sky.bind_sky_profile`, `btn.sky.set_time_of_day`, `btn.sky.set_weather_regime`, `btn.sky.bind_cloud_profile` | base-shell mandatory |
| free-camera inspection | editor shell + `108` | `btn.view.viewport`, `btn.view.outliner`, `btn.view.inspector`, `btn.view.content_browser`, `btn.view.terrain_lab`, `btn.view.material_lab`, `btn.view.sky_lab` | base-shell mandatory |
| immediate consequence preview | editor `92` / `95` | `btn.material.preview_bullet_hit`, `btn.material.preview_blast`, `btn.material.preview_wetness`, `btn.material.preview_burn`, `btn.audio.preview_audibility_free_camera` | base-shell mandatory + advanced authoring |
| save and validate | editor `108` | `btn.terrain.save_chunks`, `btn.world.save_world_package`, `btn.world.validate_world` | base-shell mandatory |

## Full-game-first extension
The material-first day-zero route is not the whole game.
After the world is materially lawful, the next conveyor must proceed through:
- population and ecology;
- tactics, navigation, and traversal consequence;
- inventory, economy, and quest/event logic;
- audio zones and runtime mix;
- presentation/runtime proof;
- save/restore proof;
- build/export/launch proof.

These later steps may use advanced authoring and lab surfaces, but they may not contradict the base shell.

## Acceptance note
The archive fails if `editor/111`, `editor/112`, `editor/106`, and `editor/110` describe different promoted controls for the same stage.

## Material-centric branch closure
The material-first sequence is incomplete unless the operator can, without leaving the canonical grammar space:
- bind physical response family coverage;
- bind visual response coverage;
- bind acoustic response coverage;
- bind light response coverage;
- inspect cheap-runtime rung and missing-coverage blockers.

## Sequential workspace strip extension
The day-zero route now sits inside one visible stage strip:

`Workspace.World -> Workspace.Simulation -> Workspace.Capture`

Meaning:
- World stage owns project/world/terrain/material/sky authoring;
- Simulation stage owns immediate preview, audibility, and consequence inspection;
- Capture stage owns compare/capture/freeze review without inventing a second shell.

The stage strip is sequential convenience only.
It may not reassign truth ownership away from editor/tooling/sdk/engine law.


## Dream-scene continuation
After the day-zero world is lawful, the canonical continuation is:
`btn.lab.open_destruction -> btn.lab.open_hydrology -> btn.lab.open_climate_theater -> btn.lab.open_society_tactics -> btn.lab.open_wound_species -> btn.lab.open_fur_certification -> btn.lab.open_photoreal_old_floor -> btn.playbook.open_first_proof`

These surfaces are not optional for the dream stack.
They are optional only for smaller games that do not claim the dream-scene target.

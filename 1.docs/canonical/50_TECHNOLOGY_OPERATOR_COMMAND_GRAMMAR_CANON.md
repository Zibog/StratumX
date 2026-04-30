# Technology Operator Command Grammar Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Provide one active command grammar for the technology-first editor surfaces.

## Allowed verb families
- `new.*`
- `open.*`
- `save.*`
- `save_as.*`
- `import.*`
- `author.*`
- `bind.*`
- `inspect.*`
- `reveal.*`
- `paint.*`
- `sculpt.*`
- `flatten.*`
- `rebuild.*`
- `capture.*`
- `compare.*`
- `simulate.*`
- `recover.*`
- `certify.*`
- `focus.*`
- `retry.*`
- `freeze.*`

## Grammar law
Commands must declare:
`verb.family.target.mode.scope`


### Required material-first verbs
- `author.material_archetype`
- `bind.surface_family`
- `bind.response_profile`
- `mutate.layer_weight`
- `mutate.biome_overlay`
- `simulate.material_response`
- `rebuild.terrain_chunks`
- `save.terrain_chunks`
- `open.terrain_chunks`

### Examples
- `import.terrain.heightmap.authoring.local`
- `paint.terrain.layer.weightmap.active`
- `flatten.terrain.patch.height.active`
- `author.material.archetype.profile.active`
- `bind.material.response_profile.instance.selected`
- `simulate.material.response.blast.preview`
- `compare.audio.route.profile`
- `reveal.trace.boundary_failure.active`
- `certify.resource.pressure.recovery`

## Registry rule
The grammar policy and the command registry are one active law.
Opaque UI-only verbs, bypass verbs, or fake success verbs are illegal.

## Forbidden verbs
- verbs with no declared route target;
- commit verbs that bypass tooling transaction law;
- fake success verbs such as `force.pass` or `pretend.ok`;
- mesh-only verbs that mutate material behavior without a registry-backed archetype or response profile.


## Base-shell promoted verb families
The root grammar explicitly covers promoted base-shell and advanced-authoring routes as first-class commands, not as informal UI conveniences:
- `validate.world`
- `focus.shell_surface`
- `smooth.terrain.patch`
- `duplicate.material_profile`
- `bind.material.microdetail_profile`
- `bind.material.weather_modulation`
- `preview.material.burn`
- `bind.sky.profile`
- `set.sky.time_of_day`
- `set.sky.weather_regime`
- `bind.sky.cloud_profile`
- `assign.audio.emitter_class_world_source`
- `bind.audio.zone_profile_world_surface`
- `bind.audio.priority_ducking_policy`
- `preview.audio.audibility_free_camera`
- `preview.audio.obstruction_vs_occlusion`
- `preview.audio.indoor_outdoor_transition`
- `inspect.audio.voice_subtitle_legality`

## Root discipline
Base-shell buttons and advanced everyday authoring controls are grammar-bearing commands.
They may not live as unnamed menu prose.

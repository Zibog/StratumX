# Terrain Layer Blend, Aftermath, And Biome Overlay Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze how terrain layers, biome litter, structural scatter, and aftermath overlays coexist so that terrain authoring is materially truthful, blend-safe, and cheap enough for world-scale use.

## Terrain authoring law
Terrain is not merely a heightfield plus pretty textures.
Terrain owns layered surface families, blend policies, biome overlays, and aftermath overlays that must remain compatible with blast, digging, wetness, fire, and structural fallout.

## Canonical terrain layer classes
| Layer class | Role |
|---|---|
| `base_layer` | primary walkable or structural terrain substance |
| `substrate_layer` | the exposed under-layer revealed by impact, wear, or cut |
| `biome_overlay` | litter, cones, pine needles, leaves, peat, moss, reeds |
| `structural_scatter_overlay` | rubble, brick dust, asphalt fragments, concrete fines |
| `aftermath_overlay` | scorch, fresh earth, mud churn, exposed aggregate, soot |
| `seasonal_or_weather_overlay` | snow, frost, standing water, dust film |

## Mandatory terrain layer fields
- `layer_id`
- `surface_family_ref`
- `material_archetype_ref`
- `response_profile_ref`
- `blend_policy_ref`
- `visual_stack_ref`
- `biome_overlay_default_refs`
- `aftermath_overlay_default_refs`
- `dominance_order`
- `runtime_mutation_allowed`

## Blend law
- all blend weights must normalize to one legal total per texel or cell;
- one dominant surface family must remain answerable for consequence law even when visual blend is smooth;
- visual blend softness may not erase the dominant consequence owner;
- aftermath overlays may reveal substrate without permanently destroying the author-time base layer identity.

## Canonical precedence order
1. base or substrate owner;
2. biome overlay;
3. structural scatter overlay;
4. aftermath overlay;
5. weather or seasonal modulation.

Later layers may modulate presentation and limited local interaction, but may not steal ownership from the dominant base/substrate owner unless a legal state transition explicitly promotes them.

## Canonical examples
### Dirt field with grass cover
- dominant owner: `surface.terrain.dirt_field`
- visual overlay: grass cover
- aftermath after blast: exposed fresh earth + crushed grass + ejecta scatter

### Asphalt road with edge dirt and gravel
- dominant owner: `surface.terrain.asphalt_road`
- border blend to `surface.terrain.gravel_track`
- aftermath after blast: cracked asphalt, edge exposure, gravel throw, dust

### Conifer forest floor
- dominant owner: `surface.terrain.conifer_floor`
- biome overlays: cones, bark, pine needles, branch litter
- aftermath after fire/blast: char, ash, exposed soil, scorched litter

## Dirty-region and chunk law
- terrain paint, sculpt, flatten, or overlay mutation must publish a dirty region;
- chunk rebuild may touch only the declared dirty region plus legal border padding;
- aftermath overlays are chunk-local by default and may not trigger whole-world rebuilds;
- layer blend truth is runtime-serializable and may not live only in renderer caches.

## Aftermath law
An aftermath overlay is not decorative if it changes future behavior.
If a crater exposes fresh earth or broken asphalt that affects traversal, wetness, burn, sound, or visibility, the aftermath overlay becomes truth-bearing state.

## Forbidden shortcuts
- painting a texture with no surface family binding;
- storing biome litter only in foliage placement with no terrain backlink;
- faking crater aftermath with decal-only presentation when traversal or material response changed;
- collapsing all mixed border behavior into one generic "blend" material.

## Required operator routes
This canon must lower through:
- editor `58` for material response and blast legality;
- editor `107` for heightmap import and canonicalization;
- editor `108` for terrain/world authoring and validation;
- editor `110` for every promoted terrain and paint button.

## Current posture
`document_gold / closes_terrain_blend_and_aftermath_gap / no_demo_content`

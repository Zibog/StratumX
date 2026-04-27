# Material Archetype And Surface Family Registry Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the canonical registry for material archetypes, surface families, and the minimum behavioral vocabulary that covers the overwhelming majority of world-facing authoring without mesh-owned law.

This document closes the gap between "a thing looks like asphalt" and "a thing owns asphalt-grade consequence law".

## Material-first ownership law
- geometry owns shape, bounds, and topology;
- a material archetype owns baseline physical and consequence law;
- a surface family owns deployment-specific semantics, blend rules, and environmental assumptions;
- a material instance stack owns the exact author-time composition used by a world object, terrain layer, runtime binding, or living host-bound layered stack;
- no mesh, prefab, or static model may smuggle hidden blast, burn, wetness, fracture, coverage, or support behavior outside this registry.

## Canonical archetype baseline set
The following archetypes are mandatory because together they cover the first 90% of believable world consequence needs.

| Archetype id | Baseline domain | Typical use |
|---|---|---|
| `archetype.earth_loam` | soil / soft terrain | exposed dirt, craters, dug ground |
| `archetype.earth_clay` | soil / cohesive terrain | sticky mud, dense ground |
| `archetype.grass_cover` | organic ground cover | grass, meadow surface |
| `archetype.conifer_litter` | organic litter | pine needles, cones, bark litter |
| `archetype.gravel` | granular hard-fill | roadside aggregate, ballast |
| `archetype.asphalt_road` | road surface | roads, paths, parking lots |
| `archetype.concrete_pour` | rigid mineral structure | poured slabs, walls, supports |
| `archetype.brick_masonry` | rigid block structure | brick walls, chimneys, masonry piers |
| `archetype.plaster_render` | brittle finish layer | facade skin, interior skim coat |
| `archetype.drywall_panel` | thin brittle interior surface | interior partitions |
| `archetype.wood_dead` | dry timber | fences, boards, pallets |
| `archetype.wood_live` | living wood | trunks, branches, roots |
| `archetype.metal_thin` | deformable sheet or light section | sheet metal, lockers, cans |
| `archetype.metal_heavy` | dense structural metal | beams, rails, armor plates |
| `archetype.glass_common` | brittle transparent surface | windows, bottles |
| `archetype.ceramic` | brittle fired mineral | tiles, sanitary ware |
| `archetype.fabric_light` | thin cloth | shirts, flags, curtains |
| `archetype.fabric_heavy` | heavy cloth | coats, tarps, upholstery |
| `archetype.leather` | dense flexible organic surface | boots, belts, straps |
| `archetype.flesh_human` | living body soft tissue | exposed tissue, skin-owned material layer |
| `archetype.bone` | rigid biological support | skeleton response |
| `archetype.foliage_mass` | leaves / shrubs | bushes, low foliage |

## Mandatory law columns for every archetype
Every canonical archetype row must freeze the following properties.

| Field | Meaning |
|---|---|
| `mass_density_class` | coarse density class used by physics, damage, and audio |
| `fracture_class` | brittle, ductile, fibrous, granular, flexible, living |
| `burn_class` | non-burning, slow-char, sustained-burn, flash-burn, smolder |
| `wetness_class` | absorbent, non-absorbent, film-wet, saturation-prone |
| `penetration_class` | low, medium, high resistance to ballistic or tool penetration |
| `blast_response_class` | crater, crack, dent, shatter, tear, scatter, scorch-only |
| `debris_class` | dust, flakes, splinters, shards, chunks, clods, litter |
| `aftermath_class` | what persistent downstream state must exist after response |
| `audio_response_class` | impact family, occlusion family, break family |
| `navigation_response_class` | passable, slippery, blocked-when-broken, noise-heavy |
| `assembly_pattern_class` | monolithic, layered, masonry-grid, plank-array, panel-grid, fibrous bundle, living tissue stack |
| `seam_density_class` | none, sparse, medium, dense, field-dense |
| `default_thickness_family_ref` | canonical thickness and shell/core expectation |
| `default_cross_section_family_ref` | canonical break-surface and inner-material reveal |
| `default_damage_mask_family_ref` | canonical crack / breach / erosion operator family |
| `default_consequence_tier` | default sleep/wake tier law from root `87` |

## Material territory law
Material law is not terrain-only.
The same canonical language must cover the following territory families.

| Territory family | Meaning | Typical owner |
|---|---|---|
| `territory.landscape` | terrain, roads, cut slopes, soil, gravel, asphalt | region / terrain cell |
| `territory.vegetation` | grass, shrubs, trees, canopy masses, roots | world vegetation instance or cluster |
| `territory.built_structure` | walls, floors, ceilings, roofs, supports, facades | structure assembly |
| `territory.prop` | movable or placeable objects, furniture, containers, tools | prop instance |
| `territory.living_surface` | skin, tissue, horn, shell, fur-bearing or feather-bearing outer layer | living model surface package |

A world is invalid if terrain uses one material vocabulary while structures, props, vegetation, or living surfaces use private response names.

## Territory method law
Different territories may execute with different cheap runtime methods while remaining under one material language.

| Territory family | Required deployment methods |
|---|---|
| `territory.landscape` | field blend, patch imprint, cheap layer aftermath, clipmap-friendly summaries |
| `territory.vegetation` | coverage clusters, burn/char progression, bend/break family, cheap ash or litter summary |
| `territory.built_structure` | seam-aware mask reveal, module-aware support loss, bounded breach, cheap collapse and occupancy update |
| `territory.prop` | local damage field, detach/settle, host absorption, cheap rigid fall then summary merge |
| `territory.living_surface` | host-bound layer traversal, outer-to-inner reveal, exposure publication, separate handoff to wound/species law |

## Surface family law
A surface family is not the same thing as an archetype.
An archetype answers "what kind of matter is this?".
A surface family answers "how is that matter deployed in this world and how may it blend with neighbors?".

## Mandatory surface-family baseline set
| Surface family id | Primary archetype | Typical deployment |
|---|---|---|
| `surface.terrain.dirt_field` | `archetype.earth_loam` | ordinary exposed soil |
| `surface.terrain.mud_field` | `archetype.earth_clay` | wet or churned soil |
| `surface.terrain.grass_field` | `archetype.grass_cover` | grass-bearing outdoor terrain |
| `surface.terrain.conifer_floor` | `archetype.conifer_litter` | pine litter, cones, bark, forest waste |
| `surface.terrain.gravel_track` | `archetype.gravel` | gravel road or shoulder |
| `surface.terrain.asphalt_road` | `archetype.asphalt_road` | paved road surface |
| `surface.struct.concrete_wall` | `archetype.concrete_pour` | concrete barriers, slabs, supports |
| `surface.struct.brick_wall` | `archetype.brick_masonry` | brick walls and facades |
| `surface.struct.plaster_finish` | `archetype.plaster_render` | outer/inner finish skin |
| `surface.struct.drywall_partition` | `archetype.drywall_panel` | thin interior partitions |
| `surface.struct.roof_tile` | `archetype.ceramic` | tiled roof and brittle roof skins |
| `surface.prop.wood_fence` | `archetype.wood_dead` | fences and simple wooden props |
| `surface.prop.wood_crate` | `archetype.wood_dead` | boxy props, pallets, crates |
| `surface.prop.live_tree` | `archetype.wood_live` | trunk/branch living tree surface |
| `surface.prop.metal_railing` | `archetype.metal_heavy` | handrails, supports |
| `surface.prop.metal_sheet` | `archetype.metal_thin` | barrels, doors, panels |
| `surface.prop.window_glass` | `archetype.glass_common` | windows and glass panes |
| `surface.prop.ceramic_tile` | `archetype.ceramic` | tiles and ceramic props |
| `surface.garment.light_cloth` | `archetype.fabric_light` | shirts, flags, cloth drape |
| `surface.garment.heavy_cloth` | `archetype.fabric_heavy` | coats, canvas, tarps |
| `surface.living.skin_human` | `archetype.flesh_human` | living human outer tissue layer |
| `surface.living.bone_exposed` | `archetype.bone` | exposed rigid biological support |

## Surface-family mandatory fields
- `surface_family_id`
- `territory_family_ref`
- `primary_archetype_ref`
- `secondary_archetype_refs` when blending is canonical
- `blend_policy_ref`
- `response_profile_ref`
- `thickness_family_ref`
- `cross_section_family_ref`
- `damage_mask_family_ref`
- `weather_modulation_profile_ref`
- `damage_visual_profile_ref`
- `coverage_profile_ref` when coverage, fur, grass, feather, brush, or hair posture is canonical
- `structural_proxy_family_ref`
- `default_biome_overlay_refs`
- `default_aftermath_overlay_refs`
- `navigation_surface_policy_ref`
- `acoustic_surface_policy_ref`
- `fragment_merge_policy_ref`

## Patterned-assembly law
Some materials are field-dense or module-dense by deployment even when the base matter is simple.
Canonical examples include:
- brick masonry with repeated joints;
- roof tile fields;
- plank walls and plank floors;
- rebar-backed concrete shells;
- layered drywall / insulation / stud assemblies;
- grass or crop carpets;
- skin + cloth + gear surface stacks on living models.

For these families the deployment law must explicitly freeze:
- `module_shape_class`;
- `joint_or_gap_policy_ref`;
- `support_proxy_family_ref`;
- `mask_repeat_or_seed_policy_ref`;
- `collapse_bias_family_ref`.

The engine may not pretend a field-dense assembly is monolithic if the response law depends on seams, joints, modules, or repeated supports.

## Coverage-overlay law
Coverage systems such as grass blades, fur, hair, feathering, brush, moss fringe, or similar dense repeated surface detail are legal only as canonical coverage overlays.
They may be authored through material law and surface families, but they may not become a hidden second truth system detached from the owning package or living host.

## Registry law
- every authorable material-bearing world thing must resolve to one archetype and one surface family;
- a material profile that skips archetype selection is invalid;
- a surface family may specialize deployment but may not silently replace the archetype's baseline consequence law;
- the registry must prefer "author one law once and reuse everywhere" over per-mesh micro-rules;
- material archetypes and surface families are freeze-relevant when they participate in compare/capture/certify routes.

## Forbidden shortcuts
- mesh-specific hidden physics knobs that bypass archetype law;
- shader-only wetness, burn, crack, crater, tissue-tear, or hair/fur semantics with no canonical state owner;
- terrain paint layers that name textures but not surface families;
- duplicate ad hoc material ids for what is semantically the same archetype.

## Required cross-document bindings
- response and coupling behavior must obey root `88`, root `96–98`, engine `50`, engine `53`, engine `71`, engine `103–106`;
- material and terrain operator routes must bind through editor `58`, `92`, `108`, `110`, and `113`;
- package serialization must obey world `50`.

## Current posture
`document_gold / closes_material_first_registry_gap / no_demo_content`

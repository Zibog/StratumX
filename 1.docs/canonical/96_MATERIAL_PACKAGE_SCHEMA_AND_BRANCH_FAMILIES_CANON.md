# Material Package Schema And Branch Families Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the canonical package schema that turns material-first law into a compact, reusable runtime bundle.

This document exists so the archive no longer treats a material as an accidental pile of shader knobs.
A lawful material package must describe:
- what the matter is;
- how it is deployed;
- how thick or layered it is;
- how it breaks, chars, wets, tears, or dents;
- how the broken inner face looks and sounds;
- how coverage systems such as grass, fur, hair, feathering, or dense brush are attached;
- how fragments sleep and disappear into host state;
- how the package degrades on old hardware without lying.

## Material package master law
A material package is the smallest reusable author-time unit that is allowed to carry world-facing material law.
It is not the same thing as a mesh, prefab, shader graph, or texture set.

A lawful package is composed of six mandatory branch groups.

| Branch group | Meaning | Mandatory refs |
|---|---|---|
| identity branch | matter identity and deployment identity | `material_archetype_ref`, `surface_family_ref`, `territory_family_ref` |
| assembly branch | shell/core thickness, seams, repeated modules, support proxy, host binding | `thickness_profile_ref`, `assembly_pattern_ref`, `support_proxy_family_ref` |
| response branch | cause-class interpretation | `response_profile_ref`, `interaction_policy_ref` |
| reveal branch | break face, hole edge, crack edge, char edge, wet reveal, coverage reveal | `cross_section_profile_ref`, `damage_mask_family_ref`, `damage_visual_profile_ref` |
| time branch | slow progression under fire, wetness, aging, residue, settle | `weather_modulation_profile_ref`, `aftermath_overlay_profile_ref`, `state_transition_policy_ref` |
| cheapness branch | downgrade, summarize, fragment merge, far-world posture, promoted-local coverage mode | `fragment_policy_ref`, `degrade_policy_ref`, `persistence_posture_ref` |

## Territory-family contract
The package must declare one territory family from root `84`.
The territory family decides what the package is allowed to couple to by default.

| Territory family | Typical coupling rights |
|---|---|
| `territory.landscape` | terrain-cell blend, crater patch, rut, wet spread, vegetation overlap |
| `territory.vegetation` | wind sway, burn spread, root support, cluster collapse, cheap ash summary |
| `territory.built_structure` | support graph, module seams, breach masks, occupancy and traversal updates |
| `territory.prop` | rigid-body motion, detach/settle, cheap host merge, inventory or interaction carrier if declared elsewhere |
| `territory.living_surface` | wound-surface carrier, tissue/cloth/armor layering, outer-surface tear and exposure without owning vital-state semantics |

## Assembly-pattern families
A lawful package must pick one assembly-pattern family.

| Assembly pattern | Meaning | Typical materials |
|---|---|---|
| `assembly.monolithic_shell` | one shell or slab with no repeated seam logic | poured concrete slab, cast metal plate |
| `assembly.shell_plus_core` | outer shell and distinct inner core | plaster over brick, bark over wood, skin over tissue |
| `assembly.layered_stack` | multiple ordered layers | drywall assembly, roof buildup, armor + cloth + tissue |
| `assembly.masonry_grid` | repeated modules with joints | brick walls, pavers, cinder blocks |
| `assembly.plank_array` | repeated strips or boards | fences, floors, pallets, doors |
| `assembly.field_dense_cover` | repeated small surface elements | grass carpets, gravel beds, leaf litter |
| `assembly.fibrous_bundle` | fibers or strands dominate failure mode | cloth, rope-like matter, fur clumps |
| `assembly.living_surface_stack` | biologically ordered outer-to-inner layers | skin, hide, shell-covered tissue stacks |

## Mandatory package fields
Every package row must expose the following fields.

- `material_package_id`
- `material_archetype_ref`
- `surface_family_ref`
- `territory_family_ref`
- `assembly_pattern_ref`
- `thickness_profile_ref`
- `seam_density_class`
- `module_shape_class`
- `support_proxy_family_ref`
- `host_binding_policy_ref` when the package is attached to a living or articulated host
- `cross_section_profile_ref`
- `damage_mask_family_ref`
- `coverage_profile_ref` when coverage or dense repeated surface detail is canonical
- `response_profile_ref`
- `fragment_policy_ref`
- `degrade_policy_ref`
- `persistence_posture_ref`

## Thickness-profile law
A thickness profile is a canonical answer to all of the following:
- how deep the top shell is;
- where the inner face begins;
- whether the package is hollow, solid, layered, porous, or field-dense;
- when a point event stays cosmetic;
- when a point event becomes perforation or split;
- when an area event becomes breach or collapse.

Thickness is legal as a compact profile or family id.
It is illegal as an unstructured rumor inside one ad hoc damage function.

## Cross-section-profile law
A cross-section profile tells the engine what becomes visible when a break is revealed without requiring full mesh surgery.
A lawful profile may define:
- inner albedo/normal/roughness family;
- edge alpha family;
- exposed fiber / aggregate / mortar / tissue family;
- soot, wetness, blood, or dirt-friendly modulation classes;
- cheap near-camera variants for edge richness.

## Coverage-profile law
A coverage profile is the canonical answer for dense repeated surface detail whose visual presence must stay tied to material law.
A lawful profile may define:
- density mask family;
- length / height class;
- direction / flow family;
- clump family;
- wind response family;
- wet / char / dirt modulation classes;
- far-summary and promoted-local richness modes.

Coverage profiles are legal for terrain grass, brush edges, moss fringe, fur, hair, feathering, and similar repeated surface detail.
Coverage profiles may not create a second hidden runtime truth owner.

## Package law for living surfaces
Living-surface packages are allowed and required.
They must remain generic engine truth, not game-script truth.
A living-surface package may define:
- outer skin or hide surface;
- under-surface tissue reveal;
- cloth / armor / fur overlays as layered stack members;
- tear / puncture / burn / char / wet response;
- fragment and residue legality.

Living-surface packages may not own:
- lethality;
- species logic;
- AI behavior;
- medical simulation verdicts.

Those stay in the wound/species/agent canons.

## Host-bound layer law
A living or articulated host may carry multiple material packages in ordered traversal.
Examples:
- coat -> shirt -> skin -> tissue -> bone;
- hide -> fat/tissue -> bone;
- armored plate -> padding -> garment -> skin -> tissue.

The ordered traversal is legal material/runtime truth.
The packages remain lawful material objects, but the living runtime owns traversal, exposure, and handoff to wound/species consequence.

## Editor binding law
The material-centric editor surface in `editor/113` must expose this package as one coherent grammar space.
An operator may inspect branches separately, but the package remains one lawful object.

## Forbidden shortcuts
- one generic package reused across semantically different territory families;
- cross-section truth that exists only as a baked screenshot;
- assembly-pattern truth that exists only inside one custom destruction script;
- living-surface packages that smuggle vital-state logic into material identity;
- fur/hair/grass systems that live only in a shader graph and have no package-level law.

## Current posture
`document_gold / closes_material_package_schema_gap`

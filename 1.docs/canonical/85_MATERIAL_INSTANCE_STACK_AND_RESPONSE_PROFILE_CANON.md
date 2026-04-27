# Material Instance Stack And Response Profile Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the exact stack that turns an archetype and a surface family into a concrete, reusable world-facing material instance.

This document exists so that an author can say "this is heavy cloth" or "this is asphalt road" once, then bind visual, behavioral, weather, and aftermath layers without moving the law into geometry, prefab hacks, or shader-only tricks.

## Canonical material instance stack
A material instance stack is ordered and may not skip ownership-bearing layers.

| Stack layer | Required | Role |
|---|---|---|
| `material_instance_ref` | yes | stable identity of the stack |
| `material_archetype_ref` | yes | baseline physical law |
| `surface_family_ref` | yes | deployment semantics and blend assumptions |
| `territory_family_ref` | yes | landscape / vegetation / structure / prop / living-surface posture |
| `response_profile_ref` | yes | hit/blast/burn/wetness consequences |
| `thickness_profile_ref` | yes | shell/core depth, puncture depth, collapse depth expectations |
| `cross_section_profile_ref` | yes | exposed inner-face, splinter face, rubble face, tissue-face, or core-face reveal |
| `damage_mask_family_ref` | yes | crack / breach / erosion / char / wet-spread operator family |
| `texture_stack_ref` | yes | albedo/normal/roughness/AO/etc. binding |
| `microdetail_profile_ref` | conditional | fabric fuzz, bark breakup, soil grain, plaster noise |
| `weather_modulation_profile_ref` | yes | wetness, dust, frost, char, soot modulation |
| `damage_visual_profile_ref` | yes | cracks, tears, holes, edge exposure, fracture reveal |
| `aftermath_overlay_profile_ref` | conditional | scorch, debris residue, exposed substrate, blood, mud |
| `acoustic_profile_ref` | conditional | impact/occlusion/leakage response |
| `interaction_policy_ref` | yes | tool, bullet, blast, fire, water interactions |
| `fragment_policy_ref` | yes | detach, fall, merge-to-host, residue posture |
| `degrade_policy_ref` | yes | legal visual/runtime downgrade ladder |

## Response profile law
A response profile is not optional once a material participates in runtime consequence.
A response profile owns the exact canonical mapping from cause classes to consequence classes.

## Mandatory response profile fields
- `response_profile_id`
- `supported_trigger_classes`
- `contact_response_family`
- `penetration_response_class`
- `ricochet_response_class`
- `blast_response_class`
- `burn_response_class`
- `wetness_response_class`
- `fracture_response_class`
- `breach_response_class`
- `debris_response_class`
- `aftermath_overlay_policy_ref`
- `state_transition_policy_ref`
- `sleep_wake_policy_ref`
- `support_topology_policy_ref`
- `compare_baseline_family`
- `capture_bundle_family`
- `certification_pack_id`

## Trigger classes that every general-purpose profile must classify
- `trigger.contact`
- `trigger.ballistic_hit`
- `trigger.ballistic_graze`
- `trigger.blast_overpressure`
- `trigger.thermal_contact`
- `trigger.fire_exposure`
- `trigger.wetness_contact`
- `trigger.tool_dig_or_cut`
- `trigger.structural_overload`
- `trigger.fall_or_collision`
- `trigger.time_degradation`

## Thickness and cross-section law
Thickness is not a mesh rumor.
Every lawful stack must expose a canonical thickness family and cross-section family so the engine can answer:
- how much depth is available before perforation;
- whether the surface is shell-only, shell-plus-core, layered, field-dense, or living tissue stack;
- what inner face becomes visible when a hole, tear, split, or breach is revealed;
- what cheap reveal is legal when the mesh itself is not rebuilt.

## Mask-operator law
A material instance stack is incomplete unless it can answer which operator family is legal for:
- point impact;
- area blast;
- edge erosion;
- burn/char progression;
- wet spread;
- splinter or crumble edge treatment.

These operators may remain cheap and procedural, but they may not be undefined.

## Layered override law
- an instance stack may override texture or microdetail content without replacing archetype law;
- an instance stack may override response thresholds only through an explicit response profile revision;
- a shader variant may not change blast, fracture, or burn meaning;
- a model or prefab may not add hidden consequence rules on top of the instance stack.

## Cheap-but-honest law
The stack exists so that a material can "sleep with knowledge".
It does not run a full solver while idle.
Instead it holds the references that let runtime wake the correct response profile when a legal trigger arrives.

## Canonical examples
### Heavy cloth coat
- archetype: `archetype.fabric_heavy`
- surface family: `surface.garment.heavy_cloth`
- response profile: cloth tear + burn + wetness + bullet-hole profile
- thickness profile: thin layered textile
- cross-section profile: frayed edge + backing-fiber reveal
- damage-mask family: tear-edge + scorch-edge operators
- aftermath overlay: soot, scorch edge, mud splash, blood, tear reveal

### Asphalt road
- archetype: `archetype.asphalt_road`
- surface family: `surface.terrain.asphalt_road`
- response profile: crack + shard + dust + crater edge exposure
- thickness profile: road-top shell + substrate depth band
- cross-section profile: broken aggregate + binder core reveal
- damage-mask family: point chip, radial crack, area spall, wet darkening
- aftermath overlay: exposed substrate, broken edge, powder dust, soot

### Brick wall
- archetype: `archetype.brick_masonry`
- surface family: `surface.struct.brick_wall`
- response profile: joint-guided crack, module loss, partial breach, rubble aftermath
- thickness profile: multi-course masonry
- cross-section profile: brick core + mortar joint reveal
- damage-mask family: joint-biased crack and breach operators
- fragment policy: limited falling modules that merge into rubble host state

## Required diagnostics
The editor/runtime stack must be able to explain:
- which archetype was active;
- which surface family and territory family were active;
- which thickness and cross-section families were active;
- which response profile answered the trigger;
- which operators were chosen for crack, breach, char, or wet-spread reveal;
- which overlays were revealed because of weather, damage, or aftermath;
- which degrade rung changed only fidelity and did not change semantics.

## Forbidden shortcuts
- storing consequence law only in mesh metadata;
- storing consequence law only in texture import metadata;
- treating a texture set as a material profile;
- using one giant generic response profile for semantically different archetypes.

## Current posture
`document_gold / closes_material_instance_gap / no_demo_content`

## Canonical response vocabulary dependency
A response profile is lawful only when every downstream branch resolves through the normalized family vocabulary in root `89`.
A stack that binds refs without naming canonical family coverage is incomplete even if the references exist.

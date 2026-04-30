# Material Instance Stack And Response Profile Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

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

---

# V34 hybrid material model closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding decision

StratumX uses a **hybrid material model**.

Runtime 1.0 materials are **profile-driven material families** with strict parameter schemas, shader variant requirements, physical/acoustic/light response references, and diagnostic proof. A visual node graph may be added later as an authoring surface, but it must compile into the same profile-family contract and cannot bypass material truth.

## Why this is binding

A full free-form node graph on day one would delay the engine, fragment shader variants, and make material-first physical response harder to validate. A profile-only model forever would limit artists later. The hybrid model keeps the first runtime fast and lawful while reserving a future graph layer.

## Runtime 1.0 material families

| Family | Required use | Core parameters | Future graph status |
|---|---|---|---|
| `terrain_pbr` | terrain layers, soil, asphalt, mud, grass | albedo, normal, roughness, AO, macro variation, layer mask refs | graph may author parameters later |
| `opaque_pbr` | buildings, props, static meshes | albedo, normal, roughness, metalness/specular, AO, emissive optional | graph may compile to family |
| `alpha_cutout` | vegetation cards, fences, thin surfaces | albedo, normal, cutout, wind class, shadow mode | graph may author foliage variants |
| `emissive` | lamps, screens, anomaly glow | emissive color/intensity, light link, bloom flag | graph may author effects |
| `water_basic` | initial puddles/water surfaces | color, normal, roughness, flow hint, depth tint | graph may expand later |
| `debug_overlay` | editor overlays | id color, opacity, channel source | no graph |
| `missing_material` | fallback | checker color, reason code | no graph |

## Material graph future rule

The future material graph is an **editor authoring compiler**, not runtime truth.

Graph output must produce:

- material family id;
- parameter block;
- texture channel bindings;
- shader variant key;
- response profile refs;
- diagnostic source map;
- fallback material row.

Graph output must not produce arbitrary backend code without variant registry approval.

## Texture channel registry

| Channel | Required interpretation |
|---|---|
| BaseColor | sRGB input; linear in shader |
| Normal | tangent-space unless family says otherwise |
| Roughness | linear scalar |
| Metalness/Specular | one model selected per family |
| AO | linear ambient occlusion |
| Emissive | linear HDR intent |
| Opacity/Cutout | alpha threshold or blend mode |
| Wetness | authored/runtime channel placeholder |
| Burn/Char | authored/runtime channel placeholder |
| Damage/Reveal | authored/runtime channel placeholder |
| MacroVariation | scale/noise/detail blend |

## Acceptance

A material is not gold unless it has:

1. family id;
2. profile schema validation;
3. texture channel verdicts;
4. shader variant key;
5. preview result;
6. missing-resource fallback;
7. response profile refs when physical/acoustic/light coupling is required;
8. capture evidence in editor/tooling.

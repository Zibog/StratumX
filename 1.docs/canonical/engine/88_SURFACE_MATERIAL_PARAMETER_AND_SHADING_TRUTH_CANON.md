# Surface Material Parameter and Shading Truth Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Own surface-level material parameters, shading families, and legal surface response inputs.

## Truth objects
MaterialTruth, SurfaceParameterBlock, ShaderFamilyRef

## Runtime phases
surface resolve -> parameter bind -> shading family select -> response publish

## Diagnostics
missing material instance, invalid parameter domain, unsupported shading family

## Exact editor entrypoints
- render-material command rows in `editor/110` via `editor/92`

## Required publications
- `obs.material.*` with material truth id, shader family, fallback verdict, and blocker code;

## Phase-4 brutal proof slices
- `material_truth_and_fallback_legality_publish_same_frame`;
- `unsupported_shader_family_emits_typed_denial`;

## Old-floor evidence obligations
- one capture bundle proving fallback legality stays explicit under pressure;
- one compare digest proving baseline/current/recovered material truth;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`

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

# Material Light Transmission And Shadow Response Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze light interaction as a strict branch of material law so transmission, diffusion, reflectance modulation, and shadow behavior are no longer scattered across graphics, atmosphere, damage, and wetness prose.

## Core law
- material law owns light interaction meaning for any material-bearing world thing;
- renderer executes the contract but does not invent light classes outside material law;
- damage, wetness, frost, soot, perforation, and exposure state may modulate light response only through canonical state-driven classes;
- cheap runtime lighting may simplify representation but must preserve the same material meaning.

## Mandatory branch groups
| Branch | Stable id prefix | Meaning |
|---|---|---|
| Transmission class | `matresp.light.transmission.*` | no-pass, thin-pass, filtered-pass, translucent-scatter, porous-leak |
| Diffusion class | `matresp.light.diffusion.*` | hard-surface, rough-diffuse, fibrous-diffuse, volumetric-soft |
| Shadow response class | `matresp.light.shadow.*` | hard-edge, broken-edge, porous-breakup, soft-body, animated-foliage |
| Damage alteration class | `matresp.light.damage.*` | crack-leak, hole-leak, soot-darken, wet-darken, char-matte, frost-bloom |
| State modulation class | `matresp.light.state.*` | clean, wet, dusty, frozen, aged, bloodied, mossed |
| Cheap ladder class | `matresp.light.cheap.*` | full, simplified BRDF, overlay-only, probe-only, static approximation |

## Required fields
Every light-response row must publish:
- `light_family_id`
- `transmission_class`
- `diffusion_class`
- `shadow_response_class`
- `reflectance_modulation_class`
- `state_modulation_refs`
- `damage_alteration_refs`
- `cheap_runtime_rungs`
- `renderer_contract_ref`
- `editor_preview_contract_ref`
- `validation_gate_family`
- `incompatibility_refs`
- `fallback_family_ref`

## Renderer contract
The graphics branch must be able to ask one material question set and get one stable answer set:
- how much light may pass;
- how much light is scattered or diffused;
- what shadow breakup the surface creates;
- how wetness / burn / fracture state modulates reflectance and transmission;
- which cheap rung is legal on old hardware or distant posture.

## Editor preview contract
The editor must preview at least:
- transmission preview
- diffusion preview
- shadow breakup preview
- damage-induced light alteration preview
- state modulation preview
- cheap-rung preview

## Incompatibility law
Examples of illegal combinations that validation must reject:
- opaque-only transmission with porous-leak shadow response;
- glass-like transmission with fibrous-only diffusion when archetype is rigid metal;
- wet-darken state modulation without wetness-capable material state;
- hole-leak damage alteration without penetration or fracture family support.

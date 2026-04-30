# Photoreal Render Pipeline Framegraph Material Lighting And Post Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define the first photoreal-capable render pipeline that StratumX must implement through the Graphics Port Layer.

## Required baseline passes
- upload/warmup pass;
- depth prepass or lawful no-depth baseline for V1;
- opaque material pass;
- sky/atmosphere pass;
- shadow pass or shadow-disabled proof path;
- local light/transient emission pass;
- post/exposure/tonemap pass;
- editor overlay pass;
- capture resolve pass.

## Material visual truth
The renderer must support material parameters for:
- base color;
- roughness;
- metallic/specular family;
- normal;
- emissive;
- opacity/cutout;
- wetness visual channel;
- char/burn visual channel;
- damage/reveal visual channel;
- material id overlay.

## Lighting baseline
Minimum photoreal seed:
- one directional light;
- one local light;
- one transient emitter class;
- one shadow tier;
- one fallback tier;
- exposure response;
- retained diagnostics.

## Post baseline
- exposure;
- tonemap;
- gamma/color-space discipline;
- debug overlay composite;
- capture resolve.

## Proof scenarios
- terrain+sky first frame;
- material visual truth frame;
- tunnel flashlight frame;
- muzzle flash transient frame;
- old-floor fallback frame.

## Current posture
`document_gold / photoreal_pipeline_defined / implementation_open`


---
# V32 Showable Pipeline Closure

## Required framegraph nodes
| Node | Required tier | Fallback |
|---|---|---|
| `clear_or_load` | T1 | debug clear |
| `sky_or_background` | T2 | sky gradient |
| `terrain_base` | T2 | proof mesh |
| `opaque_material` | T3 | missing-material shader |
| `shadow_seed` | T4 | disabled shadow verdict |
| `local_light` | T4 | capped/disabled verdict |
| `post_exposure_tonemap` | T3 | minimal LDR tonemap |
| `debug_overlay` | T2 | optional hide for final capture |
| `capture_resolve` | T1 | metadata-only capture |
| `present_or_no_present` | T1/T0 | null no-present |

## Material baseline
Required channels: base color, roughness, metallic/specular, normal, ambient occlusion placeholder, emissive, opacity/cutout, wetness placeholder, char/burn placeholder, damage/reveal placeholder, material id overlay, missing texture fallback.

## Post order
Linear scene color -> exposure -> tonemap -> output transform -> optional bloom later -> debug/editor overlay -> capture/present resolve.

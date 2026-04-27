# Graphics, Lookdev, Lighting, and Viewport Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the editor-facing workflow for materials, lighting, sky and fog conditions, viewport debug modes, and graphics budget inspection.

## Workflow
`select asset or placed entity -> inspect material and lighting-facing properties -> adjust environment or lookdev state -> request preview frame -> inspect debug overlays and budget posture -> commit or revise`

## Required surfaces
| Surface | Role |
|---|---|
| material/lookdev inspector | material class, parameters, wetness/char/dust-facing state previews |
| lighting and environment panel | lights, sky, fog, clouds, exposure, weather-facing presentation controls |
| viewport debug modes | visibility, extraction, lighting, overdraw/cost, atmosphere, material-state overlays |
| frame diagnostics panel | frame posture, degradation tier, denied or partial routes |

## Distinction law
The editor must distinguish:
- durable authored visual state;
- preview-only visual delta;
- diagnostics overlays;
- engine-published frame posture.

## Current posture
This document closes the product-side answer for graphics asks and aligns it with frame-extraction law.

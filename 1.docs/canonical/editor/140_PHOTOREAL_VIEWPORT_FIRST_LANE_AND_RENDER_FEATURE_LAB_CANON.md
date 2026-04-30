# Photoreal Viewport First Lane And Render Feature Lab Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define the editor surface for the first photoreal rendering lane: terrain+sky, material visual truth, PBR baseline, lighting/shadow, exposure, and fallback proof.

## Required stages
1. backend selected;
2. first present;
3. terrain+sky;
4. material visual truth;
5. PBR baseline;
6. lighting/shadow;
7. exposure/post;
8. tunnel/flashlight proof;
9. old-floor fallback;
10. capture/compare.

## Required controls
- toggle material id overlay;
- toggle texture residency overlay;
- toggle shadow tier overlay;
- toggle exposure debug;
- run terrain+sky capture;
- run tunnel flashlight capture;
- run muzzle flash transient capture;
- force old-floor fallback;
- compare against golden frame.

## Inspector fields
- active render feature tier;
- material visual channel completeness;
- resident texture mips;
- shadow path;
- local light path;
- post/exposure path;
- disabled fast paths;
- frame budget.

## Current posture
`document_gold / photoreal_viewport_lane_defined / implementation_open`


---
# V32 Editor Lab Closure: Showable Frame Lane

## Operator sequence
Open project -> run backend doctor -> select policy -> validate null frame plan -> start real backend present -> load proof terrain/proof mesh -> enable sky/background -> assign visual material baseline -> enable sun/local light -> enable exposure/tonemap -> capture frame -> compare/generate golden expectation -> inspect fallback badges -> freeze first visual proof artifact.

## Required checklist
Terrain/proof mesh visible, sky/background visible, material visual baseline active, lighting/shadow status resolved, output color chain active, old-floor fallback rung visible, capture/compare status retained.

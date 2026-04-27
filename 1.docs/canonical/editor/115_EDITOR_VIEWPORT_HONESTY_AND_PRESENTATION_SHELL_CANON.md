# Editor Viewport Honesty And Presentation Shell Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the primary viewport as an honest world-editing surface and define how split or detached viewports stay lawful.

## Honest primary viewport
The primary viewport must:
- show the real opened world;
- use the same lawful frame chain as runtime presentation;
- expose backend/degrade badges;
- surface capture/comparison readiness;
- explain failure in-place when the frame chain is blocked.

## Viewport cockpit law
The viewport is not an empty rectangle.
It is a cockpit-grade operator surface with four mandatory chrome layers:
- a **top viewport toolbar** for camera, transform, overlays, and mode switches;
- a **left contextual creation bar** for placement/sculpt/paint/sample/probe actions bound to the active stage;
- a **right lightweight quick-inspector ribbon** for currently hot values such as material sample, world coordinates, biome, structural stress, wetness, or audibility;
- a **bottom world timeline / event strip** for sim scrubbing, bake/job status, warnings, and recent causal incidents.

## Split-view law
Vertical and horizontal viewport splits are legal when:
- the primary viewport remains visible;
- secondary viewports consume the same world truth;
- topology and throttling posture are published;
- capture eligibility is explicit per viewport.

## Required shell badges
- backend class
- present path
- degrade rung
- capture readiness
- secondary-view throttle badge
- world/save identity
- stage identity

## Camera law
Each viewport must publish:
- camera class
- independent/shared-camera posture
- selection-follow posture
- simulation-follow posture when active

## Contextual creation bar law
The creation bar must adapt to the active stage without forking shell identity.
Examples:
- terrain stage: sculpt, flatten, smooth, paint, stamp, cut, sample;
- vegetation stage: paint, scatter, erase, cluster, explain placement;
- structures stage: place, align, foundation fit, basement carve, rupture preview;
- groups stage: stamp set, scatter set, conform, validate intersections;
- material stage: sample material, preview response, inspect where-used;
- audio stage: listener probe, occlusion probe, emitter placement, zone paint.

Stage-local tools may change labels and icons.
They may not fabricate hidden truth channels or bypass exact routes.

## World timeline law
The viewport bottom strip must support a world timeline posture that can show:
- recent simulation incidents;
- weather/front transitions;
- fire/wetness/smoke events;
- collapse or terrain-deformation events;
- bake jobs and validation failures;
- bookmarks for compare, capture, and recovery anchors.

This strip may behave differently from a cinematic timeline.
Its authority is world causality review, not sequencing edits into media output.

## Viewport controls
The shell must expose:
- orbit/pan/dolly;
- frame-selection;
- camera-speed control;
- overlay toggles;
- split-view creation/destruction;
- honest failure states;
- material sample / audio probe / physics probe / AI probe affordances;
- one-click focus on current denial source when a viewport-side action fails.

## Required buttons
- `btn.view.viewport`
- `btn.view.viewport_split_vertical`
- `btn.view.viewport_split_horizontal`

## Prohibitions
- no decorative splash cards as the main viewport;
- no debug-only renderer as the editor viewport while runtime uses another chain;
- no secondary viewport that swallows the only backend diagnostics;
- no stage where the operator must leave the viewport entirely just to understand what the world is doing.

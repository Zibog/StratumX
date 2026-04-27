# Graphics Frame and View Extraction Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the path from runtime truth to one legal frame.
It defines view extraction, visibility determination, renderable presentation data, frame ownership, and debug/publication boundaries.

## Truth boundary
Engine world truth is not the frame.
The frame is a derived publication product created from runtime truth, visibility policy, camera/view state, and graphics degradation posture.
Editor and tooling may inspect frame products, but they do not own the truth they summarize.

## Canonical extraction route
`runtime world truth -> view/camera state intake -> visibility and relevance determination -> renderable extraction -> lighting/atmosphere/material contribution assembly -> frame package publication -> viewport presentation`

## Required extracted classes
| Class | Purpose |
|---|---|
| visible_primitive_set | legal draw-facing subset for the current view |
| material_instance_inputs | resolved material-facing values for visible surfaces |
| light_and_shadow_inputs | per-frame lighting contributors and shadow-relevant summaries |
| atmosphere_and_fog_inputs | sky, fog, volumetric, cloud, wetness-facing presentation inputs |
| debug_overlay_inputs | non-authoritative inspection data for frame diagnostics |
| frame_budget_posture | explicit quality/degradation posture for the produced frame |

## Visibility law
Visibility belongs to engine-side view extraction policy.
Editor or tooling may request debug visualization, but may not override authoritative visibility truth by hidden local guesses.

## Publication law
A published frame may contain presentable render data, debug overlays, and cost posture.
It may not contain hidden mutation authority.
The viewport consumes the frame as presentation input only.

## Camera law
A frame must identify which camera or view-context produced it.
Editor, gameplay, cinematic, thumbnail, and diagnostic views are distinct legal producers with explicit handoff rules.

## Degradation law
If the engine lowers graphics fidelity, the produced frame must explicitly declare its quality or degradation posture rather than silently degrading while claiming full parity.

## Current posture
Viewport delivery and sky/environment delivery already exist as strong doc anchors.
A fully rebased frame-extraction evidence pack remains future work, so current posture is `specified_only` to `partial` depending on the consuming workflow.

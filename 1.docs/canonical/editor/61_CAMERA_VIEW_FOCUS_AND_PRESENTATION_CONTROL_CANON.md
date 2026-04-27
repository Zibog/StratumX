# Camera, View, Focus, and Presentation Control Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines how the editor names and switches camera and view contexts.
It separates editor navigation, gameplay possession, cinematic presentation, and diagnostic views.

## View classes
| View class | Purpose |
|---|---|
| editor_navigation_view | orbit, fly, focus, framing, transform-oriented work |
| gameplay_view | possessed runtime or play/simulate view |
| cinematic_view | authored sequence or presentation camera |
| diagnostic_view | overlays, probes, visibility or audio/physics debug |
| thumbnail_or_asset_view | focused preview on content rather than world runtime |

## Focus law
Focus is a named state.
The editor must be able to explain whether focus currently belongs to UI, editor navigation, gameplay possession, or a cinematic/diagnostic consumer.

## Handoff law
Switching views may change control routing and frame context.
That handoff must be explicit and reversible.

## Current posture
View and viewport law already existed in pieces.
This document closes the missing answer layer for camera and focus asks.

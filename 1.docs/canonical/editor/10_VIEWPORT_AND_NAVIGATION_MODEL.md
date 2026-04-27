# Viewport and Navigation Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the product-visible viewport as the main construction surface of the editor.
The viewport is not decorative chrome and not a local schematic canvas.
It is the product surface where lower-stack world truth becomes visible frame truth.

## Viewport law
The primary viewport must always be one of these declared postures:
- `Boot`
- `LiveAuthoring`
- `Degraded`
- `Failure`
- `PlayAttached`
- `SimulateAttached`
- `AuthoringRestored`

The viewport may never sit in an undeclared mixed posture.

## Honest-frame law
A viewport window is not considered valid just because it draws pixels.
The viewport is honest only when:
- the visible frame resolves to a lower-stack frame class;
- the world identity is known;
- terrain posture is known;
- sky/environment posture is known;
- diagnostics can explain the current state.

A local 2D schematic or decorative placeholder may exist for debug evidence.
It may not be mistaken for the product viewport.

## Navigation posture
The viewport must support:
- free camera navigation in authoring posture;
- focus selected;
- frame world;
- camera bookmarks;
- runtime-attached camera posture;
- authoring camera restoration after runtime detach.

## Render modes
The first closure requires at least:
- `Lit`
- `Unlit`
- `Wireframe`
- `TerrainDebug`
- `EnvironmentDebug`
- `PerformanceOverlay`

These are viewport presentation modes.
They do not redefine lower-stack truth.

## Mandatory overlays
The viewport must support overlays for:
- grid and transform gizmos
- selection and bounds
- terrain diagnostics
- environment diagnostics
- performance and degradation posture
- runtime attach posture

## Selection law
Selection in the viewport must resolve to the same canonical identity family as the outliner and inspector.
Viewport-only local ids are forbidden.

## Runtime-attach law
When the operator enters play or simulate, the viewport must visibly transition to a runtime-attached posture without losing declared world identity.
When the operator returns to authoring, the viewport must visibly transition back to an authoring-restored posture.

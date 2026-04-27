# overlay_and_gizmo_system Level

Canonical layer: `overlay_and_gizmo_system`
Activation class: `hot-when-visible`.

## Role
overlay and gizmo system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- gizmo rendering state, overlay bars, helper visuals, snapping and guide presentation

## Consumes
- active viewport context, tool context, diagnostics overlays, preview overlays

## Emits
- manipulator requests, snapping requests, overlay toggle intents

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- transform truth or simulation truth

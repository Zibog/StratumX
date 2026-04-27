# ui_hud_authoring_suite Level

Canonical layer: `ui_hud_authoring_suite`
Activation class: `warm-suite`.

## Role
ui hud authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- UI layout surfaces, HUD composition views, style/theme inspectors, responsive previews

## Consumes
- UI/HUD projections, inspector, preview hooks, validation hooks

## Emits
- UI/HUD edit requests, layout requests, preview requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- UI truth

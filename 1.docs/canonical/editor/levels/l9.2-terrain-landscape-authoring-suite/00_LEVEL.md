# terrain_landscape_authoring_suite Level

Canonical layer: `terrain_landscape_authoring_suite`
Activation class: `warm-suite`.

## Role
terrain landscape authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- sculpt, smooth, flatten, stamp, erosion-like tools, layer paint, biome masks, foliage context, terrain diagnostics

## Consumes
- terrain projections, material/lookdev projections, viewport, preview and validation hooks

## Emits
- terrain and landscape edit requests, material mask requests, preview requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- terrain truth

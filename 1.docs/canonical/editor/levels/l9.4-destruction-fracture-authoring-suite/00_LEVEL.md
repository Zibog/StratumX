# destruction_fracture_authoring_suite Level

Canonical layer: `destruction_fracture_authoring_suite`
Activation class: `warm-suite`.

## Role
destruction fracture authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- fracture tools, clustering, structural link views, stress and collapse previews, destruction diagnostics

## Consumes
- destruction projections, material projections, preview hooks, validation hooks

## Emits
- fracture edit requests, structural link requests, destruction preview requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- destruction truth

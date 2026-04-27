# material_lookdev_authoring_suite Level

Canonical layer: `material_lookdev_authoring_suite`
Activation class: `warm-suite`.

## Role
material lookdev authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- material editors, lookdev stages, decal and post-process authoring views, lighting presets, material diagnostics

## Consumes
- asset/material projections, viewport previews, inspector, validation hooks

## Emits
- material parameter requests, stage preview requests, lookdev edit requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- material truth

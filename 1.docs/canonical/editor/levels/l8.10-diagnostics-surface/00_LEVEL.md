# diagnostics_surface Level

Canonical layer: `diagnostics_surface`
Activation class: `warm-aux`.

## Role
diagnostics surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- warnings/errors panels, metrics and traces views, budget strip, health dashboards

## Consumes
- diagnostics streams, validation findings, budget/runtime signals

## Emits
- inspection requests, filter requests, reveal requests, capture requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- diagnostic source truth

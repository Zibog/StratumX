# assistant_surface Level

Canonical layer: `assistant_surface`
Activation class: `warm-aux`.

## Role
assistant surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- assistant dock, proposal panel, evidence views, apply/revert controls, progress widgets

## Consumes
- assistant runtime projections from L6A/L7A, focused context hooks, diagnostics hints

## Emits
- assistant invocation requests, accept/reject/apply/revert requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- assistant runtime truth or planning truth

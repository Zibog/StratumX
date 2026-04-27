# collaboration_session_surface Level

Canonical layer: `collaboration_session_surface`
Activation class: `session-surface`.

## Role
collaboration session surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- presence views, shared session projections, participant lists, session-scoped awareness widgets

## Consumes
- collaboration metadata, review state, diagnostics hints

## Emits
- session join/leave/invite requests, share-view requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- editor truth

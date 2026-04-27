# asset_gate_and_approval_surface Level

Canonical layer: `asset_gate_and_approval_surface`
Activation class: `session-surface`.

## Role
asset gate and approval surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- approval queues, gate summaries, policy check surfaces, owner/reviewer assignment views

## Consumes
- validation results, package/build status, collaboration metadata

## Emits
- approve/reject/request-changes intents, policy rerun requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- approval policy truth outside legal lower-stack hooks

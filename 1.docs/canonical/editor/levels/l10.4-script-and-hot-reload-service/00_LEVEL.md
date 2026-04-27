# script_and_hot_reload_service Level

Canonical layer: `script_and_hot_reload_service`
Activation class: `warm-service`.

## Role
script and hot reload service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- script bridge registration, hot reload requests, script compile/reload status, runtime-safe handoff surfaces

## Consumes
- package/dependency service, project integration hooks, diagnostics

## Emits
- reload requests, compile requests, state transition projections

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- core runtime truth

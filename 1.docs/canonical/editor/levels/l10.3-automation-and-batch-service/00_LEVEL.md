# automation_and_batch_service Level

Canonical layer: `automation_and_batch_service`
Activation class: `batch-service`.

## Role
automation and batch service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- batch task runners, automation schedules, headless task orchestration, queue state

## Consumes
- suite requests, build/release requests, project bootstrap requests, diagnostics

## Emits
- batch task submissions, task state, result projections

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- hidden orchestration truth

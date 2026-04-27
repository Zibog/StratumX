# production_dashboard_and_traceability Level

Canonical layer: `production_dashboard_and_traceability`
Activation class: `cold-ops`.

## Role
production dashboard and traceability is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- dashboards, milestone and asset status surfaces, provenance links, pipeline traceability views

## Consumes
- service status, approval/gate metadata, diagnostics, build/release metadata

## Emits
- report and reveal requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- production truth outside sourced projections

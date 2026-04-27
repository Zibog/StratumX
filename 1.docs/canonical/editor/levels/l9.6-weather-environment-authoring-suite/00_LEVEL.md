# weather_environment_authoring_suite Level

Canonical layer: `weather_environment_authoring_suite`
Activation class: `warm-suite`.

## Role
weather environment authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- sky and time-of-day controls, weather front tools, wind/fog/rain volumes, environment probes, environment diagnostics

## Consumes
- environment projections, viewport, preview hooks, validation hooks

## Emits
- environment edit requests, weather and probe requests, preview requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- environment truth

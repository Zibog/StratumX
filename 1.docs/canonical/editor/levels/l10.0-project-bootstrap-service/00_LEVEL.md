# project_bootstrap_service Level

Canonical layer: `project_bootstrap_service`
Activation class: `cold-service`.

## Role
project bootstrap service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- project templates, baseline workspace scaffolds, starter configuration generation, bootstrap status

## Consumes
- template registry, package dependency service, lower-stack project hooks

## Emits
- project creation requests, scaffold application events

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- project truth after bootstrap

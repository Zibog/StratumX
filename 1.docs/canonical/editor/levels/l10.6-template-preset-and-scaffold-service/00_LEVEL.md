# template_preset_and_scaffold_service Level

Canonical layer: `template_preset_and_scaffold_service`
Activation class: `cold-service`.

## Role
template preset and scaffold service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- preset catalogs, scene/world/tool templates, scaffold packs, starter content bindings

## Consumes
- project bootstrap service, package service, content browser requests

## Emits
- preset application requests, scaffold generation requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- truth after application

# quest_event_logic_authoring_suite Level

Canonical layer: `quest_event_logic_authoring_suite`
Activation class: `warm-suite`.

## Role
quest event logic authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- trigger and event graph surfaces, quest chain views, condition/action authoring, test harnesses

## Consumes
- graph service projections, world/scene context, validation hooks

## Emits
- quest/event/logic edit requests, graph requests, preview/test requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- quest/event truth

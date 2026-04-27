# interaction_routing_system Level

Canonical layer: `interaction_routing_system`
Activation class: `hot-when-focused`.

## Role
interaction routing system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- focused interaction routing, priority arbitration, pointer/key/gesture dispatch normalization

## Consumes
- viewport focus, tool context, shortcut definitions, command palette hooks

## Emits
- normalized editor intents, focus transitions, route-denied events

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- hidden command execution or direct truth mutation

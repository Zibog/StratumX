# graph_authoring_service Level

Canonical layer: `graph_authoring_service`
Activation class: `warm-service`.

## Role
graph authoring service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- generic graph model hosts, node schema registry, graph validation hooks, graph serialization helpers

## Consumes
- suite graph requests, diagnostics, package/dependency service

## Emits
- graph job requests, validation events, schema queries

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- graph truth owned by lower stack or project-specific rules

# tool_context_system Level

Canonical layer: `tool_context_system`
Activation class: `warm-context`.

## Role
tool context system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- global and suite-local tool contexts, active tool mode markers, transient manipulator state, domain tool shelves

## Consumes
- active selection presentation, active viewport context, suite activation state

## Emits
- tool mode switches, tool-local requests, contextual overlay requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- lower-stack truth or hidden domain state

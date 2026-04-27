# workspace_layout_system Level

Canonical layer: `workspace_layout_system`
Activation class: `warm-shell`.

## Role
workspace layout system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- saved layouts, window docking descriptors, per-workspace panel arrangements, detached host descriptors

## Consumes
- shell frame presence, panel registry, surface availability

## Emits
- layout save/load/apply requests, workspace switch requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- panel truth, suite truth

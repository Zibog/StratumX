# build_release_surface Level

Canonical layer: `build_release_surface`
Activation class: `warm-aux`.

## Role
build release surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- build graph panel, package targets, release presets, checklist views, artifact status projections

## Consumes
- build/release service status, artifact projections, validation results

## Emits
- build/run/package requests, release checklist requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- packaging logic or artifact truth

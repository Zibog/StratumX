# review_annotation_surface Level

Canonical layer: `review_annotation_surface`
Activation class: `session-surface`.

## Role
review annotation surface is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- comments, annotations, markup overlays, review threads, approval request links

## Consumes
- review metadata, viewport/outliner/content/inspector context

## Emits
- comment/review requests, resolve requests, reveal requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- world truth

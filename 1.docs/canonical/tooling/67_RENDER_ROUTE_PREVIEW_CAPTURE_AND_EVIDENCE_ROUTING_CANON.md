# Render Route Preview Capture And Evidence Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact routing law for render preview, capture, baseline comparison, and evidence append.

## Intent families
- `intent.render.preview`
- `intent.render.capture`
- `intent.render.compare`
- `intent.render.capture_recovered`

## Required route stages
1. validate preview/capture scope
2. resolve frame scope and viewset
3. resolve backend and present posture
4. request frame/capture from lower layers
5. publish blocker or success summary
6. append retained artifact when required
7. return focus to viewport, capture rail, or blocker pane

## Cache and invalidation
Declared caches:
- frame preview cache
- capture metadata cache
- compare card cache

Invalidation triggers:
- backend class change
- shader target change
- world/sky/material scope change
- layout topology change when capture surface changes

## Retry and rollback
- preview retries are bounded and never hide the first blocker;
- capture retries must retain the first failed artifact when a recovered run is attempted;
- compare routes never mutate truth.

## Artifact law
Capture and compare routes must publish:
- active backend class
- present path
- degrade rung
- first blocker code
- reason trace ref

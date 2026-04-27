# viewport_system Level

Canonical layer: `viewport_system`
Activation class: `hot-viewport`.

## Owns
- viewport camera state
- viewport mode and overlay presentation state
- manipulator presentation state
- transform and placement tool UI state
- focused hit-test and framing state

## Consumes
- world and entity projections
- preview outputs
- validation overlays
- runtime-attach projections where active

## Emits
- selection requests
- transform/placement/world-tool intents
- preview requests
- runtime attach intents

## Never owns
- committed world truth
- preview truth
- build/package truth

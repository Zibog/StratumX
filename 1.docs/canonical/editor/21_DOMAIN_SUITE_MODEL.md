# Domain Suite Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

`L9` hosts broad universal domain authoring suites rather than project-specific game packs.
Each suite may be rich, but must remain bounded by universal authoring concerns.

## Required suites
- L9.0 world
- L9.1 scene/entity
- L9.2 terrain/landscape
- L9.3 material/lookdev
- L9.4 destruction/fracture
- L9.5 simulation/AI
- L9.6 weather/environment
- L9.7 animation/cinematics
- L9.8 audio/voice
- L9.9 UI/HUD
- L9.10 quest/event/logic
- L9.11 build/validation/release

## First product-result priority law
The first product-result closure does not require every suite to be equally hot.

### Hot for first closure
- `L9.0 world`
- `L9.1 scene/entity`
- `L9.2 terrain/landscape`
- `L9.3 material/lookdev`
- `L9.6 weather/environment`
- `L9.11 build/validation/release`

### Warm for first closure
- `L9.7 animation/cinematics`
- `L9.8 audio/voice`

### Cold for first closure
- `L9.4 destruction/fracture`
- `L9.5 simulation/AI`
- `L9.9 UI/HUD`
- `L9.10 quest/event/logic`

## First product-result suite anchors
### `L9.0 world`
- world identity
- world-open intent
- startup/reference world selection
- partition and region posture

### `L9.2 terrain/landscape`
- terrain presence and walkable surface
- terrain patch visibility in viewport
- terrain authoring and material binding
- terrain diagnostics visibility

### `L9.6 weather/environment`
- sky/environment binding
- environment preset authoring
- time-of-day preview controls
- diagnostics-visible environment posture

### `L9.7 animation/cinematics`
For first closure this suite remains warm.
It does not block the terrain + sky + walk loop.
Its first mandatory next-wave responsibility is interaction-driven motion authoring and debug exposure defined in `49_INTERACTION_DRIVEN_MOTION_AND_CONSTRAINT_CANON.md`.

### `L9.11 build/validation/release`
- world-open legality
- viewport readiness diagnostics
- runtime-entry legality
- degraded-mode explanation
- host and contract mismatch reporting

## Suite law
Suites may publish requests, presets, selections, preview controls, and diagnostics overlays.
They may not own lower-stack simulation truth, render truth, or shadow state.

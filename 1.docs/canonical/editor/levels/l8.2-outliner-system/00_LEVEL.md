# outliner_system Level

Canonical layer: `outliner_system`
Activation class: `warm-panel`.

## Owns
- hierarchy/tree presentation state
- filter, sort, and expansion state
- selection presentation state for the panel
- context-menu state
- region/cell and layer browsing state

## Consumes
- hierarchy, layer, region, and runtime-loaded projections
- validation badges and dirty/diverged posture
- source-control status where legal

## Emits
- select/focus requests
- create/rename/delete/duplicate intents
- prefab/layer/chunk intents
- reveal/pin/copy intents

## Never owns
- hierarchy truth
- prefab truth
- layer truth

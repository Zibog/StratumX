# inspector_system Level

Canonical layer: `inspector_system`
Activation class: `warm-panel`.

## Owns
- inspector tab/pin state
- staged-edit state
- foldout/group expansion state
- compare/diff view state
- batch-edit form state

## Consumes
- detail projections
- validation verdicts
- preview results
- runtime-inspection projections
- prefab override and asset import projections

## Emits
- legal staged-edit requests
- prefab apply/revert/unpack intents
- validate/bake intents
- runtime diff requests

## Never owns
- committed entity/component truth
- prefab truth
- asset import/build truth

# Dependencies

## Legal dependencies
- scene-entity suite
- world suite
- content browser
- inspector system
- tooling index/derived surfaces

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `outliner_system` and may not be widened by analogy to neighboring levels.

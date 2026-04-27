# Dependencies

## Legal dependencies
- outliner system
- inspector system
- content browser
- plugin host
- tooling scene families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `scene_entity_authoring_suite` and may not be widened by analogy to neighboring levels.

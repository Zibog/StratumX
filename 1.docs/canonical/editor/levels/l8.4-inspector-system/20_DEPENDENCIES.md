# Dependencies

## Legal dependencies
- scene-entity suite
- content browser
- validation_runtime
- plugin-and-extension-host
- tooling inspection views

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `inspector_system` and may not be widened by analogy to neighboring levels.

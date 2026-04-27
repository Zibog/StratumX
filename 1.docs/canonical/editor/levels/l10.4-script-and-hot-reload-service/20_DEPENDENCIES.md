# Dependencies

## Legal dependencies
- content browser
- plugin host
- tooling validation/runtime/assistant families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `script_and_hot_reload_service` and may not be widened by analogy to neighboring levels.

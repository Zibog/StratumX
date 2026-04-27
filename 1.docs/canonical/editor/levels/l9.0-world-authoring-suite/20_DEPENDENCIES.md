# Dependencies

## Legal dependencies
- viewport system
- outliner system
- world tools
- tooling world families
- build-validation-release suite

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `world_authoring_suite` and may not be widened by analogy to neighboring levels.

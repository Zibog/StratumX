# Dependencies

## Legal dependencies
- viewport system
- tool context system
- world suite
- tooling terrain/material families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `terrain_landscape_authoring_suite` and may not be widened by analogy to neighboring levels.

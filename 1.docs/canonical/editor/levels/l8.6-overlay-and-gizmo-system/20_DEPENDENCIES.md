# Dependencies

## Legal dependencies
- viewport system
- tool context system
- plugin host
- tooling preview/stream surfaces

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `overlay_and_gizmo_system` and may not be widened by analogy to neighboring levels.

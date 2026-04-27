# Dependencies

## Legal dependencies
- viewport system
- diagnostics surface
- build-release surface
- tooling preview/stream/validation families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `playtest_and_capture_operations` and may not be widened by analogy to neighboring levels.

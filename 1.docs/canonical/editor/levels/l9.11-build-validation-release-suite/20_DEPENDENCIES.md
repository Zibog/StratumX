# Dependencies

## Legal dependencies
- build-release surface
- diagnostics surface
- tooling build/release/validation families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `build_validation_release_suite` and may not be widened by analogy to neighboring levels.

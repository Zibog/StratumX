# Dependencies

## Legal dependencies
- tooling build_runtime
- tooling release_runtime
- package/dependency service
- asset gate and approval surface

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `build_release_surface` and may not be widened by analogy to neighboring levels.

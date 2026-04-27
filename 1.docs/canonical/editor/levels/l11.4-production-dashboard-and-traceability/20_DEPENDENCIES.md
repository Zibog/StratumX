# Dependencies

## Legal dependencies
- diagnostics surface
- build-release surface
- tooling reporting/meta families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `production_dashboard_and_traceability` and may not be widened by analogy to neighboring levels.

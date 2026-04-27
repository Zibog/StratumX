# Dependencies

## Legal dependencies
- workspace layout system
- template/preset service
- package/dependency service
- tooling project meta families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `project_bootstrap_service` and may not be widened by analogy to neighboring levels.

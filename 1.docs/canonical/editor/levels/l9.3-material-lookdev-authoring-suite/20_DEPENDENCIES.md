# Dependencies

## Legal dependencies
- content browser
- viewport system
- import-export pipeline
- tooling material/render families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `material_lookdev_authoring_suite` and may not be widened by analogy to neighboring levels.

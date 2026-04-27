# Dependencies

## Legal dependencies
- content browser
- viewport system
- production dashboard

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `review_annotation_surface` and may not be widened by analogy to neighboring levels.

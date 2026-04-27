# Dependencies

## Legal dependencies
- content browser
- build-release surface
- tooling build/artifact/validation families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `import_export_pipeline_service` and may not be widened by analogy to neighboring levels.

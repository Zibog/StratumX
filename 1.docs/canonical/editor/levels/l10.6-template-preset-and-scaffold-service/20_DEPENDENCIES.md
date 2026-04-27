# Dependencies

## Legal dependencies
- project bootstrap service
- content browser
- tooling content/meta families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `template_preset_and_scaffold_service` and may not be widened by analogy to neighboring levels.

# Dependencies

## Legal dependencies
- interaction routing system
- overlay and gizmo system
- viewport system
- tooling activation sidecars

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `tool_context_system` and may not be widened by analogy to neighboring levels.

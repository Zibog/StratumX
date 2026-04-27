# Dependencies

## Legal dependencies
- editor shell
- workspace schema and migration model
- autosave recovery model

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `workspace_layout_system` and may not be widened by analogy to neighboring levels.

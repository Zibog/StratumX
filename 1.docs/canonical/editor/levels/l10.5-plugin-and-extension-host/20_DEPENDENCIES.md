# Dependencies

## Legal dependencies
- editor shell
- inspector system
- assistant surface
- tooling assistant/validation families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `plugin_and_extension_host` and may not be widened by analogy to neighboring levels.

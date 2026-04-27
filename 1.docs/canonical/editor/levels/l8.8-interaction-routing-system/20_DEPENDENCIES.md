# Dependencies

## Legal dependencies
- viewport system
- content browser
- inspector system
- tool context system
- assistant surface

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `interaction_routing_system` and may not be widened by analogy to neighboring levels.

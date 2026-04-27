# Dependencies

## Legal dependencies
- viewport system
- inspector system
- quest-event-logic suite
- tooling population/combat/simulation families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `simulation_ai_authoring_suite` and may not be widened by analogy to neighboring levels.

# Dependencies

## Legal dependencies
- content browser
- quest-event-logic suite
- ui suite
- plugin host

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `graph_authoring_service` and may not be widened by analogy to neighboring levels.

# Dependencies

## Legal dependencies
- plugin host
- package/dependency service
- tooling automation/meta families

## Forbidden dependencies
- undeclared lower-stack private internals
- runtime or tooling truth ownership that belongs elsewhere
- hidden side channels bypassing commands, requests, or declared refs

## Dependency law
The dependency contour above is specific to `automation_and_batch_service` and may not be widened by analogy to neighboring levels.

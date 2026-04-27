# Dependencies

This contract belongs specifically to the safety gates level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6a.0-assistant-sessions`
- `l6.9-budget-runtime`
- `l6.7-stream-plane`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `safety_gates` and may not be widened by analogy.

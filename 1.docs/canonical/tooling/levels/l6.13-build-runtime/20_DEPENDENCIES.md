# Dependencies

This contract belongs specifically to the build runtime level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6.6-artifact-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`
- `l6.11-validation-runtime`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `build_runtime` and may not be widened by analogy.

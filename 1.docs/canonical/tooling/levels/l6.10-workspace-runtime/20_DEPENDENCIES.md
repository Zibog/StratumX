# Dependencies

This contract belongs specifically to the workspace runtime level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6.0-authority-core`
- `l6.3-snapshot-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`
- `l6.1-tool-selection`
- `l6.2-tool-focus-refs`
- `l6.16-tool-panel-refs`
- `l6.17-tool-view-refs`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `workspace_runtime` and may not be widened by analogy.

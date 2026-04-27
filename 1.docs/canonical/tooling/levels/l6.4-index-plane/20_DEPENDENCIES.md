# Dependencies

This contract belongs specifically to the index plane level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6.3-snapshot-plane`
- `l6.6-artifact-plane`
- `l6.8-cache-plane`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `index_plane` and may not be widened by analogy.

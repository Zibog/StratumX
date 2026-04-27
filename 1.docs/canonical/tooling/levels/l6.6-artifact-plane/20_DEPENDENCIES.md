# Dependencies

This contract belongs specifically to the artifact plane level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6.3-snapshot-plane`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `artifact_plane` and may not be widened by analogy.

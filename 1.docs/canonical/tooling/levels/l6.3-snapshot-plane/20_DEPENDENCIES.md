# Dependencies

This contract belongs specifically to the snapshot plane level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6.0-authority-core`
- `l6.2-transaction-ledger`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `snapshot_plane` and may not be widened by analogy.

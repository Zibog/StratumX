# Dependencies

This contract belongs specifically to the transaction ledger level and may not be reused verbatim by another tooling level.


## Legal dependencies
- `l6.0-authority-core`
- `l6.1-command-envelopes`

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes or sidecars
- lower-layer truth that belongs to another semantic class

## Dependency law
The dependency contour above is specific to `transaction_ledger` and may not be widened by analogy.

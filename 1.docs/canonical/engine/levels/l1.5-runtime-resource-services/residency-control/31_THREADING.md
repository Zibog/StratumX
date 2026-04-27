# Threading

## Ownership

No global threading ownership.

## Posture

Parallel residency accounting with ordered visibility publication under runtime control.

## Parallel Units

Cells, chunks, resource pages, and residency batches.

## Write Model

Publishes residency truth local to the service; does not mutate world truth.

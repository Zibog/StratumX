# Threading

## Ownership

No global threading ownership.

## Posture

Parallel request build with ordered activation publication under runtime control.

## Parallel Units

Cells, chunks, region batches, and IO request batches.

## Write Model

Publishes activation/eviction decisions; does not mutate authoritative world truth.

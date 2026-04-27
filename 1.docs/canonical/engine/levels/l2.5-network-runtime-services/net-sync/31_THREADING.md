# Threading

## Ownership

No global threading ownership.

## Posture

Parallel dirty scan and delta build with ordered per-connection snapshot publication.

## Parallel Units

Region interest batches, connection batches, snapshot batches, and delta batches.

## Write Model

Publishes staged/exported sync products only; does not mutate authoritative world truth.

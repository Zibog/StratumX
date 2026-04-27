# Threading

## Ownership

No global threading ownership.

## Posture

Parallel local prediction and history capture with ordered correction and reconcile publication.

## Parallel Units

Owned-input batches, history buffers, rewind query batches, and correction batches.

## Write Model

Publishes prediction/correction-local products only; never becomes authoritative world truth.

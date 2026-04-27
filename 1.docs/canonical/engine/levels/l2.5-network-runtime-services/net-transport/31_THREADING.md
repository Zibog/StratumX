# Threading

## Ownership

No global threading ownership.

## Posture

Parallel encode/decode with ordered publication per connection and lane.

## Parallel Units

Connection batches, packet batches, and lane batches.

## Write Model

Owns transport-local buffers only; does not own replicated state truth.

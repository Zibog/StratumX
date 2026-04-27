# Threading

## Ownership

No global threading ownership.

## Posture

Parallel-ready by independent model tasks and batched requests under runtime control.

## Parallel Units

Inference tasks, batched requests, normalization jobs.

## Write Model

Produces structured outputs; world application remains above this crate.

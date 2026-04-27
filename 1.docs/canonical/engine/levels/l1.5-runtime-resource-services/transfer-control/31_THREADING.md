# Threading

## Ownership

No global threading ownership.

## Posture

Pipeline-parallel decode/staging/upload work under runtime and budget law.

## Parallel Units

Decode jobs, staging blocks, upload batches, and fence-bound release batches.

## Write Model

Owns transfer-local state only; does not own world mutation or rendering policy.

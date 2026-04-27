# Threading

## Ownership

No global threading ownership.

## Posture

Parallel-ready staging with controlled final apply ownership elsewhere.

## Parallel Units

Independent batched mutation sets.

## Write Model

Staged mutation only; final application happens above this crate.

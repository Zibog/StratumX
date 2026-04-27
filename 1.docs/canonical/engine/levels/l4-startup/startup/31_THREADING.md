# Threading

## Ownership

No global threading ownership.

## Posture

Bootstrap and launch layer; may initialize runtime-facing resources but does not own execution parallelism.

## Parallel Units

Not a hot execution layer.

## Write Model

Startup configures and launches; it does not own world mutation.

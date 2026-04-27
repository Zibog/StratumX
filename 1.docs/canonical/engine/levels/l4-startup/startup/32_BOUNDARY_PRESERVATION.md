# Boundary Preservation

## Canonical Position

`engine_startup` is preserved as a standalone crate and moved from earlier L3 to canonical `L4`.

## Reason

Startup is the highest launch layer. It is not a peer of execution, simulation, or synthesis systems.

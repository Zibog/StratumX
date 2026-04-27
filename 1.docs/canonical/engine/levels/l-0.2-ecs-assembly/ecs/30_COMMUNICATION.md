# Communication

## Inputs

Lower ECS substrate crates and base contracts.

## Outputs

Assembled ECS boundary, query-facing access substrate, and world-facing ECS surface.

## Canonical Data Forms

- assembled ECS surface
- query surface
- mutation boundary

## Upward Flow

`engine_ecs` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

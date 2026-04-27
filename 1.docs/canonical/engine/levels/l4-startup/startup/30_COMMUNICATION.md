# Communication

## Inputs

Configuration, profile selection directives, world assembly inputs, family registration inputs, prepared resource inputs, and runtime-pack inputs.

## Outputs

Launched runtime instance, wired family set, validated runtime-pack inputs, startup diagnostics, initialized execution environment, and narrow public bridge export surfaces for `L5`.

## Canonical Data Forms

- config
- profile selection
- runtime instance
- startup diagnostics
- public bridge export surface

## Upward Flow

`engine_startup` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.
External `L5` bridge bind is legal only through the named public export surfaces.

# Communication

## Inputs

Activation requests, world/region context, runtime budgets, and startup-selected runtime-pack inputs.

## Outputs

Activation decisions, prefetch decisions, eviction requests, and IO scheduling products.

## Canonical Data Forms

- descriptors
- request batches
- result batches
- diagnostics

## Upward Flow

`engine_stream_control` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

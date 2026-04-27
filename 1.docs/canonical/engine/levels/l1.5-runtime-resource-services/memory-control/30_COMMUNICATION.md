# Communication

## Inputs

Runtime budget windows, allocation requests, and pressure notifications.

## Outputs

Allocator results, pool state, reclaim requests, and memory diagnostics.

## Canonical Data Forms

- descriptors
- request batches
- result batches
- diagnostics

## Upward Flow

`engine_memory_control` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

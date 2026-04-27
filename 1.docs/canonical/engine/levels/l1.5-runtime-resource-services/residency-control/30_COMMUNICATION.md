# Communication

## Inputs

Activation decisions, eviction requests, runtime handles, and memory-pressure signals.

## Outputs

CPU/GPU residency maps, refcount products, and pressure reports.

## Canonical Data Forms

- descriptors
- request batches
- result batches
- diagnostics

## Upward Flow

`engine_residency_control` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

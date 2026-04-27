# Communication

## Inputs

Authoritative world snapshots, region/context visibility, dirty state, and transport-lane descriptors.

## Outputs

Interest sets, snapshot batches, delta batches, ack products, and sync diagnostics.

## Canonical Data Forms

- descriptors
- request batches
- staged/exported products
- diagnostics

## Upward Flow

`engine_net_sync` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

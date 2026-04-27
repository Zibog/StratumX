# Communication

## Inputs

Connection requests, input packets, control packets, state packets, and transport policy descriptors.

## Outputs

Ordered packet publication, receive batches, lane-pressure diagnostics, and connection events.

## Canonical Data Forms

- descriptors
- request batches
- staged/exported products
- diagnostics

## Upward Flow

`engine_net_transport` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

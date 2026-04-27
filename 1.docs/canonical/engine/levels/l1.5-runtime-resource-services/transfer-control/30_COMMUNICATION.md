# Communication

## Inputs

Startup-selected runtime-pack inputs, activation decisions, and staging/backing requests.

## Outputs

Decode jobs, staging products, upload batches, and release-fence products.

## Canonical Data Forms

- descriptors
- request batches
- result batches
- diagnostics

## Upward Flow

`engine_transfer_control` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

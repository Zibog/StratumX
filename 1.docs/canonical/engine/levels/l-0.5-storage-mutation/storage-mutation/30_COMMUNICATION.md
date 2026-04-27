# Communication

## Inputs

Access windows, handles, and layout-aware mutation targets.

## Outputs

Deferred writes, change sets, and apply payloads.

## Canonical Data Forms

- mutation buffers
- change sets
- apply payloads

## Upward Flow

`engine_storage_mutation` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

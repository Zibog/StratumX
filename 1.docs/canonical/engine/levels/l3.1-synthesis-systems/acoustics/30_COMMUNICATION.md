# Communication

## Inputs

World snapshots, ECS-extracted acoustic scene data, material descriptors, spatial context, runtime-issued synthesis windows, residency products, and transfer products.

## Outputs

Propagation products, occlusion products, acoustic outputs, and diagnostics.

## Canonical Data Forms

- propagation products
- occlusion products
- acoustic outputs
- diagnostics

## Upward Flow

`engine_acoustics` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

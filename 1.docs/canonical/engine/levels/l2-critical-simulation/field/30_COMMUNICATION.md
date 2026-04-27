# Communication

## Inputs

World snapshots, region partitions, spatial descriptors, and material descriptors.

## Outputs

Field deltas, region-local world changes, structural field products, and atmosphere/terrain/fluid/thermal outputs.

## Canonical Data Forms

- field deltas
- region deltas
- structural products
- atmospheric products

## Upward Flow

`engine_field` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

# Communication

## Inputs

World snapshots, material descriptors, spatial context, region partitions, and runtime execution windows.

## Outputs

Local motion updates, contact results, trajectory results, impact products, and staged world deltas.

## Canonical Data Forms

- contact products
- motion deltas
- trajectory products
- impact products

## Upward Flow

`engine_kinetics` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.

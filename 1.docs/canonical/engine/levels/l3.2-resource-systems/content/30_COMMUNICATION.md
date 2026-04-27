# Communication

## Inputs

Source inputs, transform descriptors, metadata descriptors, material descriptors, and region/chunk descriptors.

## Outputs

Prepared resource products, runtime-ready pack products, streaming chunk products, and diagnostics.

## Canonical Data Forms

- ingest products
- transform products
- metadata descriptors
- runtime-ready pack products
- streaming chunk products

## Upward Flow

`engine_content` communicates upward through the listed canonical data forms rather than by hidden ownership transfer. `engine_startup` or equivalent external preparation selects which runtime-pack inputs become legal `L1.5` inputs.

# Chunk Model

## Role

Canonical world chunk grain.

## Canonical Definition

`Chunk Model` is a canonical element of `engine_world_region` inside `L-0.05. World Region`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Canonical Grain

Numeric grain is frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

| Item | Canonical value |
|---|---|
| chunk edge length | 32 m equivalent world-space edge |
| vertical subdivision slab height | 16 m |
| default same-tick halo width | 1 chunk |
| maximum explicit field-solve halo width | 2 chunks |
| same-tick dirty propagation width | 1 halo |
| cross-chunk destruction spill ceiling | <= 1 halo per authoritative tick |
| activation migration ceiling | <= 4 chunk ownership moves per authority step |
| metadata migration ceiling | <= 256 KiB metadata and <= 2,048 membership remaps per activation step |

Chunk shape is cubic for locality and addressing purposes. Alternate authored world representations must still project into this canonical chunk grain.
Interior or underground authored space must still project into `(chunk_x, chunk_y, slab_z)` addressing.


Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §4

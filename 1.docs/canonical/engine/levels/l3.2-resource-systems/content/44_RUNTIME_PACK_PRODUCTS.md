# Runtime Pack Products

## Role

Startup-bindable runtime-pack products.

## Canonical Definition

`Runtime Pack Products` is a canonical element of `engine_content` inside `L3.2. Resource Systems`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Versioned runtime-pack products with frozen pack schema, seekable pages, locality-aware chunk alignment, and corruption-recovery semantics.
GPU-ready compressed texture products and multi-frame seekable compressed pack products are canonical-compatible.

## Fixed Product Matrix

| Product class | Canonical runtime status |
|---|---|
| GPU textures | GPU-ready or transcode-ready supercompressed pages only |
| geometry buffers | startup-ready or background-prepared binary pages only |
| material/shader metadata | startup-only or background-cached preparation only |
| physics/navigation/collision/destruction substrate | cooked binary only |
| streaming audio pages | seekable compressed pages, acoustics-owned decode only |

## Illegal Patterns

- runtime raw source decode on low-latency presentation path;
- runtime cook of physics or navigation products;
- pack pages that violate canonical page size or locality law.

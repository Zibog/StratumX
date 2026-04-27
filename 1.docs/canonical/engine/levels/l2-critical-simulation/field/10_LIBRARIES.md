# Libraries

## External Library Baseline

| Library | Role in `engine_field` |
|---|---|
| `glam` | Spatial math for field solve and gradients. |
| `serde` | Serialization for field descriptors and state products. |
| `bitflags` | Compact field state flags. |
| `smallvec` | Low-allocation field batch metadata. |
| `tracing` | Family diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_field` and stays aligned to its pressure axis.

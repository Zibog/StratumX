# Libraries

## External Library Baseline

| Library | Role in `engine_storage_layout` |
|---|---|
| `smallvec` | Low-allocation descriptor and local buffer support. |
| `bitflags` | Compact layout and mode flags. |
| `serde` | Serialization for layout descriptors where required. |

## Fit

Each listed dependency serves the primary role of `engine_storage_layout` and stays aligned to its pressure axis.

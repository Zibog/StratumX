# Libraries

## External Library Baseline

| Library | Role in `engine_memory_control` |
|---|---|
| `parking_lot` | Low-overhead synchronization for pools and arenas. |
| `smallvec` | Compact allocator descriptors. |
| `tracing` | Allocation and pressure diagnostics. |
| `bumpalo` | Frame- and tick-scratch arena allocation. |
| `mimalloc` | General-purpose allocator option outside strict steady-state lanes. |

## Fit

Each listed dependency serves the primary role of `engine_memory_control` and stays aligned to its pressure axis.

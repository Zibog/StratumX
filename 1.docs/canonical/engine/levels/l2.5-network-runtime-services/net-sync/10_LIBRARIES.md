# Libraries

## External Library Baseline

| Library | Role in `engine_net_sync` |
|---|---|
| `serde` | Snapshot and delta serialization descriptors. |
| `bitflags` | Sync flags. |
| `smallvec` | Compact sync batches. |
| `tracing` | Sync diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_net_sync` and stays aligned to its pressure axis.

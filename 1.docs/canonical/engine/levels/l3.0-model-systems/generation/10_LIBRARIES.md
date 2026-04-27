# Libraries

## External Library Baseline

| Library | Role in `engine_generation` |
|---|---|
| `serde` | Structured generation products. |
| `serde_json` | Structured interchange for generation payloads where required. |
| `thiserror` | Typed generation boundary errors. |
| `smallvec` | Low-allocation generation job batches. |
| `tracing` | Generation diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_generation` and stays aligned to its pressure axis.

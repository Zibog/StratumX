# Libraries

## External Library Baseline

| Library | Role in `engine_inference` |
|---|---|
| `serde` | Structured request and response contracts. |
| `serde_json` | Model-facing structured payload format where required. |
| `thiserror` | Typed inference boundary errors. |
| `smallvec` | Low-allocation request batching. |
| `tracing` | Inference diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_inference` and stays aligned to its pressure axis.

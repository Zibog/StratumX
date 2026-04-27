# Libraries

## External Library Baseline

| Library | Role in `engine_startup` |
|---|---|
| `tracing` | Startup tracing hooks. |
| `tracing-subscriber` | Tracing initialization. |
| `serde` | Configuration parsing. |
| `serde_json` | Structured config payloads where required. |
| `backtrace` | Startup and bootstrap diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_startup` and stays aligned to its pressure axis.

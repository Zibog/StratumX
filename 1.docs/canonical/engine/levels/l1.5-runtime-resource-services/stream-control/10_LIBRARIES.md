# Libraries

## External Library Baseline

| Library | Role in `engine_stream_control` |
|---|---|
| `crossbeam` | Queue coordination primitives. |
| `smallvec` | Compact request and locality metadata. |
| `tracing` | Stream diagnostics. |

## Adjuncts

- Linux `io_uring` backends are legal optional acceleration paths.
- Seekable zstd page products are legal canonical inputs.

## Fit

Each listed dependency serves the primary role of `engine_stream_control` and stays aligned to its pressure axis.

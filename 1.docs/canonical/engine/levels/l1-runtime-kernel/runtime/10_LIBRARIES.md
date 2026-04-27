# Libraries

## External Library Baseline

| Library | Role in `engine_runtime` |
|---|---|
| `tracing` | Execution observability. |
| `rayon` | Baseline CPU parallel orchestration. |
| `crossbeam` | Coordination primitives. |
| `crossbeam-deque` | Runtime-owned worker/injector/stealer deque pattern where legal. |
| `parking_lot` | Low-overhead synchronization where justified. |
| `smallvec` | Low-allocation descriptors and batched runtime metadata. |

## Adjuncts

- `criterion` is legal for microbench regression gates.
- `loom` is legal for concurrency permutation tests over runtime-owned primitives.
- `tracy-client` is legal for explicit frame and phase markers.

## Fit

Each listed dependency serves the primary role of `engine_runtime` and stays aligned to its pressure axis.

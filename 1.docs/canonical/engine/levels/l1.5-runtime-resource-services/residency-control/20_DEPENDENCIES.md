# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_residency_control` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable resource and runtime handles. |
| `engine_world` | World truth boundary. |
| `engine_world_region` | Region and chunk substrate. |
| `engine_stream_control` | Stream activation and eviction decisions. |
| `engine_memory_control` | Runtime memory classes and pressure signals. |

## Downward Pattern

`engine_residency_control` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/resource boundaries beneath its role.

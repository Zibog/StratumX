# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_memory_control` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable runtime handles. |
| `engine_storage_layout` | Storage and page layout descriptors. |
| `engine_world` | World lifetime anchors. |
| `engine_runtime` | Runtime legality windows and budget classes. |

## Downward Pattern

`engine_memory_control` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/resource boundaries beneath its role.

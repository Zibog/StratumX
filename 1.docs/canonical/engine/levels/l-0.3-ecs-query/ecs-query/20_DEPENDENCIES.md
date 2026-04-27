# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_ecs_query` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_identity` | Identity primitives. |
| `engine_handle` | Stable references. |
| `engine_storage_access` | Access windows and views. |
| `engine_ecs_registry` | Structural membership truth. |

## Downward Pattern

`engine_ecs_query` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

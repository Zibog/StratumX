# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_ecs_registry` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_identity` | Identity primitives. |
| `engine_handle` | Stable references. |
| `engine_storage_layout` | Storage shape context. |

## Downward Pattern

`engine_ecs_registry` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

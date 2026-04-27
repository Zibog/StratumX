# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_runtime` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable references. |
| `engine_ecs` | ECS substrate. |
| `engine_world` | World truth boundary. |
| `engine_world_region` | Region substrate for partition-aware scheduling. |

## Downward Pattern

`engine_runtime` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

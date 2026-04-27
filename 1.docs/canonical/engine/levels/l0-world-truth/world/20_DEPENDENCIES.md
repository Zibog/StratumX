# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_world` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable references. |
| `engine_ecs` | ECS substrate. |
| `engine_world_spatial` | Spatial substrate. |
| `engine_world_region` | Region substrate. |

## Downward Pattern

`engine_world` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

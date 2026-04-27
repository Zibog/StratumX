# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_agents` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_ecs` | ECS substrate. |
| `engine_world` | World truth boundary. |
| `engine_world_region` | Region substrate. |
| `engine_runtime` | Runtime legality windows and phase ownership. |
| `engine_memory_control` | Allocation class and pressure boundary for family-local working sets. |

## Downward Pattern

`engine_agents` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

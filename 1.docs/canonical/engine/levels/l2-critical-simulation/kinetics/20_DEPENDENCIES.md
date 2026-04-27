# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_kinetics` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_material` | Shared world property substrate. |
| `engine_world_spatial` | Spatial substrate. |
| `engine_world_region` | Partition substrate. |
| `engine_runtime` | Runtime legality windows and phase ownership. |
| `engine_memory_control` | Allocation class and pressure boundary for family-local working sets. |

## Downward Pattern

`engine_kinetics` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

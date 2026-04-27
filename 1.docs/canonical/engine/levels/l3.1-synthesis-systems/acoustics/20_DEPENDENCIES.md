# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_acoustics` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_ecs` | ECS substrate. |
| `engine_material` | Shared world property substrate. |
| `engine_world_spatial` | Spatial substrate. |
| `engine_world_region` | Region substrate. |
| `engine_residency_control` | Runtime residency control. |
| `engine_transfer_control` | Runtime transfer control. |

## Downward Pattern

`engine_acoustics` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/resource boundaries beneath its role.

# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_world_region` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world_spatial` | Spatial substrate. |

## Downward Pattern

`engine_world_region` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

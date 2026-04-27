# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_inference` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_ecs` | ECS substrate. |

## Downward Pattern

`engine_inference` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

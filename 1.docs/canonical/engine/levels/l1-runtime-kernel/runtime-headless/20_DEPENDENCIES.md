# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_runtime_headless` depends on it |
|---|---|
| `engine_runtime` | Execution constitution. |
| `engine_world` | World truth boundary. |
| `engine_ecs` | ECS substrate. |

## Downward Pattern

`engine_runtime_headless` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_generation` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_inference` | Inference boundary for model-backed generation. |

## Downward Pattern

`engine_generation` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

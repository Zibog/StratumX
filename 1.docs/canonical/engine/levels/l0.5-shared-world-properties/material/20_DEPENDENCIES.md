# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_material` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable references where material instances are addressed. |
| `engine_world` | World-bound material context. |

## Downward Pattern

`engine_material` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

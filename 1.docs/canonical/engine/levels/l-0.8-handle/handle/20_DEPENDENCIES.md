# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_handle` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_identity` | Identity domains and typed ids. |

## Downward Pattern

`engine_handle` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

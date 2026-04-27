# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_identity` depends on it |
|---|---|
| `engine_core` | Base types and contracts. |

## Downward Pattern

`engine_identity` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

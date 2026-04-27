# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_world_spatial` depends on it |
|---|---|
| `engine_core` | Base types and contracts. |
| `engine_identity` | Identity primitives. |
| `engine_handle` | Stable references where spatially indexed. |

## Downward Pattern

`engine_world_spatial` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

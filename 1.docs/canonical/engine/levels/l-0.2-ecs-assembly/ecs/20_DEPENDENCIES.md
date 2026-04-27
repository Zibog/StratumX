# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_ecs` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_identity` | Identity primitives. |
| `engine_handle` | Stable references. |
| `engine_storage_layout` | Storage structure. |
| `engine_storage_access` | Access windows. |
| `engine_storage_mutation` | Mutation staging. |
| `engine_ecs_registry` | Structural truth. |
| `engine_ecs_query` | Traversal model. |

## Downward Pattern

`engine_ecs` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

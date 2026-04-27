# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_storage_mutation` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable references. |
| `engine_storage_layout` | Layout structure. |
| `engine_storage_access` | Legal access windows. |

## Downward Pattern

`engine_storage_mutation` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.

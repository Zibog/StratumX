# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_content` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_material` | Shared world property descriptors for runtime-ready products. |
| `engine_world_region` | Region/chunk descriptors for runtime pack products. |

## Downward Pattern

`engine_content` depends downward only on crates that supply lower contracts, lower substrates, or the required resource-processing boundaries beneath its role.

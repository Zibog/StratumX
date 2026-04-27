# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_net_sync` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_identity` | Stable entity and object identity. |
| `engine_handle` | Stable runtime/object handles. |
| `engine_world` | Authoritative world boundary. |
| `engine_world_region` | Region and cell relevance anchors. |
| `engine_runtime` | Runtime legality windows and ordered publication law. |
| `engine_net_transport` | Ordered transport publication. |

## Downward Pattern

`engine_net_sync` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/network boundaries beneath its role.

# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_net_latency` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable runtime and object handles. |
| `engine_world` | World truth boundary for validation. |
| `engine_world_region` | Region and cell anchors for rewind windows. |
| `engine_runtime` | Runtime legality windows and ordered correction law. |
| `engine_net_transport` | Ordered transport publication. |

## Downward Pattern

`engine_net_latency` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/network boundaries beneath its role.

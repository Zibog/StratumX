# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_net_transport` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable connection and runtime handles. |
| `engine_runtime` | Runtime legality windows and ordered publication law. |

## Downward Pattern

`engine_net_transport` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/network boundaries beneath its role.

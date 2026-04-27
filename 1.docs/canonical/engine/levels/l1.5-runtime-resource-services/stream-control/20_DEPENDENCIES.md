# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_stream_control` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable runtime resource and object handles. |
| `engine_world` | World truth boundary. |
| `engine_world_region` | Region and chunk substrate. |
| `engine_runtime` | Runtime legality windows and budgets. |

## Product Input Rule

Runtime-pack inputs are legal startup-mediated or externally prepared inputs. They do not create a direct dependency on `engine_content`.

## Downward Pattern

`engine_stream_control` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/resource boundaries beneath its role.

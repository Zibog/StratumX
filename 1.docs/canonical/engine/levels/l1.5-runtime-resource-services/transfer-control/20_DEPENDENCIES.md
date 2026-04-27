# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_transfer_control` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_handle` | Stable runtime resource handles. |
| `engine_stream_control` | Stream activation decisions. |
| `engine_memory_control` | Staging and backing memory classes. |

## Product Input Rule

Runtime-pack inputs are legal startup-mediated or externally prepared inputs. They do not create a direct dependency on `engine_content`.

## Downward Pattern

`engine_transfer_control` depends downward only on crates that supply lower contracts, lower substrates, or the required runtime/resource boundaries beneath its role.

# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| runtime_handle | EngineRuntimeHandle | required | stable opaque runtime surface handle | must remain opaque to upper layers |
| runtime_class | RuntimeHandleClass | required | closed runtime surface class enum | must use declared enum |
| session_handle | EngineSessionHandle | required | session that owns or attached the runtime surface | must resolve through `engine_session_handles` |
| attachment_scope | AttachmentScope | required | scope within which the runtime handle is valid | must be explicit and bounded |
| status | RuntimeHandleStatus | required | active/stale/revoked state | must transition only by declared rules |

## No hidden store law
All semantic truth in `engine_runtime_handles` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.

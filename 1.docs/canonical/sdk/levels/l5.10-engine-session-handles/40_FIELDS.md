# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| session_handle | EngineSessionHandle | required | stable opaque session handle | must never expose engine-private ids |
| session_scope_id | SessionScopeId | required | scope boundary for the session | must remain stable for the handle lifetime |
| origin_class | SessionOriginClass | required | origin of the session such as transport or local attach | must use declared enum |
| status | SessionHandleStatus | required | open/draining/closed state | must use declared enum and transition monotonically |
| issued_at_tick | IssueTick | required | issuance clock for the handle | must not change after issuance |

## No hidden store law
All semantic truth in `engine_session_handles` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.

# Engine Session Handles Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| session_handle | EngineSessionHandle | required | stable opaque session handle | must never expose engine-private ids |
| session_scope_id | SessionScopeId | required | scope boundary for the session | must remain stable for the handle lifetime |
| origin_class | SessionOriginClass | required | origin of the session such as transport or local attach | must use declared enum |
| status | SessionHandleStatus | required | open/draining/closed state | must use declared enum and transition monotonically |
| issued_at_tick | IssueTick | required | issuance clock for the handle | must not change after issuance |

## Local invariant rule
Each field above exists because `engine_session_handles` must publish engine session handles without absorbing adjacent semantic truth.

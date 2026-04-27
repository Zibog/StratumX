# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| object_handle | EngineObjectHandle | required | stable opaque object handle | must never encode engine memory address |
| identity_ref | EngineIdentityRef | required | identity ref that resolves the public object identity | must resolve through `engine_identity_refs` |
| session_handle | EngineSessionHandle | required | session that is allowed to resolve this handle | must resolve through `engine_session_handles` |
| kind | ObjectHandleKind | required | closed object-kind enum | must use declared enum only |
| status | ObjectHandleStatus | required | active/stale/revoked state | must transition only by declared rules |

## No hidden store law
All semantic truth in `engine_object_handles` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.

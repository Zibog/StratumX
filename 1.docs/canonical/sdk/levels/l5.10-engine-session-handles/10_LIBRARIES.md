# Libraries

## Local record classes
- `EngineSessionHandle` for `session_handle`
- `SessionScopeId` for `session_scope_id`
- `SessionOriginClass` for `origin_class`
- `SessionHandleStatus` for `status`
- `IssueTick` for `issued_at_tick`

## Shared registries consumed
- `compat_profiles` registry or lookup surface

## Library law
`engine_session_handles` may introduce only record classes that help publish or resolve engine session handles. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

# Libraries

## Local record classes
- `EngineRuntimeHandle` for `runtime_handle`
- `RuntimeHandleClass` for `runtime_class`
- `EngineSessionHandle` for `session_handle`
- `AttachmentScope` for `attachment_scope`
- `RuntimeHandleStatus` for `status`

## Shared registries consumed
- `engine_session_handles` registry or lookup surface

## Library law
`engine_runtime_handles` may introduce only record classes that help publish or resolve engine runtime handles. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

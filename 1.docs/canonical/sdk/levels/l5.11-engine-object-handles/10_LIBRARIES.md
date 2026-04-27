# Libraries

## Local record classes
- `EngineObjectHandle` for `object_handle`
- `EngineIdentityRef` for `identity_ref`
- `EngineSessionHandle` for `session_handle`
- `ObjectHandleKind` for `kind`
- `ObjectHandleStatus` for `status`

## Shared registries consumed
- `engine_session_handles` registry or lookup surface
- `engine_identity_refs` registry or lookup surface

## Library law
`engine_object_handles` may introduce only record classes that help publish or resolve engine object handles. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

# Libraries

## Local record classes
- `EngineIdentityRef` for `identity_ref`
- `IdentityClass` for `identity_class`
- `ExternalIdentityName` for `external_name`
- `IdentityVisibilityScope` for `visibility_scope`
- `IdentityRefStatus` for `status`

## Shared registries consumed
- `engine_session_handles` registry or lookup surface

## Library law
`engine_identity_refs` may introduce only record classes that help publish or resolve engine identity refs. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

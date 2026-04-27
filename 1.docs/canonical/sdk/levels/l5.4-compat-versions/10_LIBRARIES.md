# Libraries

## Local record classes
- `CompatVersionId` for `compat_version_id`
- `WireVersion` for `wire_version`
- `SchemaGeneration` for `schema_generation`
- `CompatVersionId` for `supersedes_version_id`
- `SupportState` for `support_state`

## Shared registries consumed
- `compat_profiles` registry or lookup surface

## Library law
`compat_versions` may introduce only record classes that help publish or resolve compatibility versions. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

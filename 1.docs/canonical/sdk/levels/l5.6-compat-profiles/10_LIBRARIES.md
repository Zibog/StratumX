# Libraries

## Local record classes
- `CompatProfileId` for `profile_id`
- `ProfileName` for `profile_name`
- `CompatVersionSet` for `allowed_version_set`
- `CompatCapabilitySet` for `capability_set`
- `CompatProfileId` for `fallback_profile_id`

## Shared registries consumed
- `compat_versions` registry or lookup surface
- `compat_capabilities` registry or lookup surface

## Library law
`compat_profiles` may introduce only record classes that help publish or resolve compatibility profiles. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

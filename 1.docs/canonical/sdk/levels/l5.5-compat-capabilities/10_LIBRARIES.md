# Libraries

## Local record classes
- `CompatCapabilityId` for `capability_id`
- `CapabilityName` for `capability_name`
- `CompatProfileId` for `availability_profile_id`
- `CapabilityDefaultState` for `default_state`
- `CapabilityDeprecationNote` for `deprecation_note`

## Shared registries consumed
- `compat_profiles` registry or lookup surface
- `compat_versions` registry or lookup surface

## Library law
`compat_capabilities` may introduce only record classes that help publish or resolve compatibility capabilities. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

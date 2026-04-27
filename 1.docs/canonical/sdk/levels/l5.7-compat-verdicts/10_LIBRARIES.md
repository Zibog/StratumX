# Libraries

## Local record classes
- `CompatVerdictId` for `compat_verdict_id`
- `CompatProfileId` for `evaluated_profile_id`
- `CompatVersionId` for `evaluated_version_id`
- `CompatVerdictState` for `verdict_state`
- `CompatReasonCodeSet` for `reason_code_set`

## Shared registries consumed
- `compat_profiles` registry or lookup surface
- `compat_versions` registry or lookup surface
- `compat_capabilities` registry or lookup surface

## Library law
`compat_verdicts` may introduce only record classes that help publish or resolve compatibility verdicts. It may not invent a parallel semantic vocabulary for adjacent bridge classes.

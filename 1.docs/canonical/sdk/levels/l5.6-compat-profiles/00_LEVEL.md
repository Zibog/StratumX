# Compatibility Profiles

## Role
`compat_profiles` bundles versions, capabilities, and support posture into named bridge compatibility profiles.

## Owns
- only the declared `compat_profiles` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_versions`
- `compat_capabilities`

## Emits
- publishes immutable compatibility profiles used by packets, controls, and transport policies

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

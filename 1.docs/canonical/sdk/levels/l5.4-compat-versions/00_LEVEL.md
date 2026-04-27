# Compatibility Versions

## Role
`compat_versions` declares protocol versions and structural compatibility bands that every bridge envelope must name.

## Owns
- only the declared `compat_versions` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_profiles`

## Emits
- publishes immutable version rows consumed by packets, controls, and metrics
- never emits mutable runtime state

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

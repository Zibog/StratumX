# Compatibility Verdicts

## Role
`compat_verdicts` records compatibility decisions derived from versions, capabilities, and profiles without mutating those source registries.

## Owns
- only the declared `compat_verdicts` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_profiles`
- `compat_versions`
- `compat_capabilities`

## Emits
- receives evaluation inputs from legality/transport validation
- publishes verdict records for downstream acceptance or rejection

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

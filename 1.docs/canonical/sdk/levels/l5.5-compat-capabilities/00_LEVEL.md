# Compatibility Capabilities

## Role
`compat_capabilities` declares named capability bits and feature switches visible across the bridge boundary.

## Owns
- only the declared `compat_capabilities` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_profiles`
- `compat_versions`

## Emits
- publishes capability rows consumed by legality gates and ingress control validation

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

# Legality Gates

## Role
`legality_gates` declares explicit allow/warn/deny gate classes that every mutating ingress control must pass before acceptance.

## Owns
- only the declared `legality_gates` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_capabilities`
- `compat_profiles`
- `compat_verdicts`

## Emits
- publishes gate rows consumed during ingress control admission
- emits explicit deny/warn codes, never hidden side effects

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

# Engine Runtime Handles

## Role
`engine_runtime_handles` provides opaque handles for public runtime surfaces such as simulation channels, subscriptions, and attachment scopes.

## Owns
- only the declared `engine_runtime_handles` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_session_handles`

## Emits
- publishes opaque runtime handles consumed by ingress controls and egress publications

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

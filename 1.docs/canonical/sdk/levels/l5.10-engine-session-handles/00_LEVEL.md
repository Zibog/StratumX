# Engine Session Handles

## Role
`engine_session_handles` provides stable opaque handles for public bridge sessions without leaking engine internals.

## Owns
- only the declared `engine_session_handles` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_profiles`

## Emits
- publishes opaque session handles consumed by packets, controls, and egress batches

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

# Engine State Refs

## Role
`engine_state_refs` publishes immutable references to engine state snapshots that may be observed but not mutated by the bridge.

## Owns
- only the declared `engine_state_refs` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_runtime_handles`
- `engine_identity_refs`

## Emits
- publishes immutable state refs for observations and diagnostics

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

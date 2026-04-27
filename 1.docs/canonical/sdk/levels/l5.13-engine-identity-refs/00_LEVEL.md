# Engine Identity Refs

## Role
`engine_identity_refs` publishes stable public identity refs that map bridge-facing identity to engine-owned entities without leaking internals.

## Owns
- only the declared `engine_identity_refs` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_session_handles`

## Emits
- publishes public identity refs for object handles, observations, and save/load references

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

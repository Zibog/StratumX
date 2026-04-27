# Engine Object Handles

## Role
`engine_object_handles` provides stable opaque handles for addressed engine objects visible through public bridge controls or observations.

## Owns
- only the declared `engine_object_handles` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_session_handles`
- `engine_identity_refs`

## Emits
- publishes opaque object handles used by ingress controls and egress batches

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

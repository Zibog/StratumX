# Engine Artifact Refs

## Role
`engine_artifact_refs` publishes immutable references to engine-produced artifacts such as captures, compiled bundles, or replay manifests.

## Owns
- only the declared `engine_artifact_refs` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_runtime_handles`
- `engine_state_refs`

## Emits
- publishes immutable artifact refs for captures, replays, and exported records

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

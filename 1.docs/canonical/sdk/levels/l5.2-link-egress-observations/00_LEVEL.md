# Link Egress Observations

## Role
`link_egress_observations` publishes factual observation batches from engine-facing snapshots back toward public L4 read surfaces.

## Owns
- only the declared `link_egress_observations` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_runtime_handles`
- `engine_state_refs`
- `transport_policies`
- `compat_profiles`

## Emits
- consumes immutable factual snapshots
- publishes read-only observation batches
- emits no mutating control intent

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

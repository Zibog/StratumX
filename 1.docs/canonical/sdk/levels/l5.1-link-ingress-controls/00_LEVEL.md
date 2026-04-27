# Link Ingress Controls

## Role
`link_ingress_controls` normalizes mutating control envelopes that travel from public L4 control surfaces into bridge-legal engine-facing command classes.

## Owns
- only the declared `link_ingress_controls` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_versions`
- `compat_capabilities`
- `transport_policies`
- `legality_gates`
- `engine_session_handles`
- `engine_runtime_handles`
- `engine_object_handles`

## Emits
- receives control requests from L4 public control APIs
- publishes accepted control envelopes for downstream authority handling
- emits deterministic rejection reasons for unsupported or illegal control classes

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

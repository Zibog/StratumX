# Link Ingress Packets

## Role
`link_ingress_packets` accepts raw ingress packet envelopes from public L4 entry points before they are split into controls, facts, or version handshakes.

## Owns
- only the declared `link_ingress_packets` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_versions`
- `transport_policies`
- `legality_gates`
- `engine_session_handles`

## Emits
- receives opaque ingress packets from public L4 transport receivers
- publishes accepted packet envelopes to split/route stages
- emits bounded reject records when policy or legality fails

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

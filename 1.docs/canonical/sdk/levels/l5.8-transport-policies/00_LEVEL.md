# Transport Policies

## Role
`transport_policies` declares bounded transport, batching, retry, and framing policies visible to public bridge traffic.

## Owns
- only the declared `transport_policies` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `compat_versions`
- `compat_profiles`

## Emits
- publishes immutable transport rules consumed by packet ingress and egress publication

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

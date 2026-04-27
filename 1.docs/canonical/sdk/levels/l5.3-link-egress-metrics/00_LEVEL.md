# Link Egress Metrics

## Role
`link_egress_metrics` publishes bounded metric series and counters for public monitoring and telemetry consumers.

## Owns
- only the declared `link_egress_metrics` records listed in `40_FIELDS.md`
- publication order and legality metadata local to this semantic class
- no gameplay, editor, or tooling truth outside the declared bridge records

## Consumes
- `engine_runtime_handles`
- `compat_profiles`
- `transport_policies`

## Emits
- receives counters/gauges already reduced by engine exports
- publishes metric batches for dashboards and monitoring APIs
- emits explicit quality flags when sampling degrades

## Gold law
This level is valid only when a reviewer can tell, without reinterpretation, which bridge records belong here and which adjacent records must stay out.

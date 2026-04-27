# tool_diagnostics_views communication

Ingress:
- diagnostics event streams
- filter/grouping change requests
- issue refresh triggers from build or validation

Egress:
- diagnostic view ids and issue-set refs
- filter digest publications
- freshness epoch updates

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.

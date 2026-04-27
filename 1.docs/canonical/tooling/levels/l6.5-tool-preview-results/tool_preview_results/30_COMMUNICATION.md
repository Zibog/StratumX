# tool_preview_results communication

Ingress:
- preview runtime completion publications
- preview cancellation completions
- preview invalidation refreshes

Egress:
- preview result refs keyed by preview_result_id
- result-state transitions
- freshness epoch updates

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.

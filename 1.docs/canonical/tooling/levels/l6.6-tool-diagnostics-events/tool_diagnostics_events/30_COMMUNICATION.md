# tool_diagnostics_events communication

Ingress:
- validation findings
- build/release errors and warnings
- runtime bridge fault publications

Egress:
- diagnostic event rows keyed by diagnostic_event_id
- severity and scope publications
- ordered issue streams

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.

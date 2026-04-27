# tool_panel_refs communication

Ingress:
- panel open/close publications
- dock/undock changes
- host migration or split-view changes

Egress:
- panel ref rows keyed by panel_ref_id
- hosting surface publications
- panel epoch updates

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.

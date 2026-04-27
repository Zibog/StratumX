# tool_activation_rules communication

Ingress:
- tool registration
- package/feature activation changes
- workspace capability changes

Egress:
- activation rule rows keyed by activation_rule_id
- required surface sets
- deny condition sets

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.

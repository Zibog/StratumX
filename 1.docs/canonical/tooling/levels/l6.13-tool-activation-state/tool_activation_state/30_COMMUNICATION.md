# tool_activation_state communication

Ingress:
- evaluated activation rules
- workspace capability changes
- tool enable/disable requests

Egress:
- activation state rows keyed by activation_state_id
- resolved rule links
- state epoch publications

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.

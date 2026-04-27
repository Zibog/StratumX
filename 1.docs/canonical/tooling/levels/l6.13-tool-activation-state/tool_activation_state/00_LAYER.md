# tool_activation_state

Role class: published tooling sidecar fact set

Canonical role:
own active activation state for tools and modes after activation rules are evaluated.

One data kind law:
this local pack owns only `tool_activation_state` publications and the fields declared for them.

Minimal operational meaning:
publish active activation state for tools and modes after activation rules are evaluated so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
rule ownership, tool execution payloads, layout ownership, selection/focus truth.

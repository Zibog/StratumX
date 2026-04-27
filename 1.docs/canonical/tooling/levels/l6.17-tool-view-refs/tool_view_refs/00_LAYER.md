# tool_view_refs

Role class: published tooling sidecar fact set

Canonical role:
own view refs for view-host coordination without taking ownership of rendered editor views.

One data kind law:
this local pack owns only `tool_view_refs` publications and the fields declared for them.

Minimal operational meaning:
publish view refs for view-host coordination without taking ownership of rendered editor views so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
rendered framebuffer or widget truth, camera/control state owned by editor, preview payload truth, selection or focus ownership.

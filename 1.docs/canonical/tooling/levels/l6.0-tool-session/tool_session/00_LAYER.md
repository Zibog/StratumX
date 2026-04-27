# tool_session

Role class: published tooling sidecar fact set

Canonical role:
own public tool-session identity, lifecycle, and caller scope shared by all downstream sidecar traffic.

One data kind law:
this local pack owns only `tool_session` publications and the fields declared for them.

Minimal operational meaning:
publish public tool-session identity, lifecycle, and caller scope shared by all downstream sidecar traffic so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
selection ownership, panel or view ownership, preview/build/release result truth, authority mutation payloads.

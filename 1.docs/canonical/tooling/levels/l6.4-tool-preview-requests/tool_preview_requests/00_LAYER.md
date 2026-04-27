# tool_preview_requests

Role class: published tooling sidecar fact set

Canonical role:
own preview requests flowing from editor/tools into preview_runtime.

One data kind law:
this local pack owns only `tool_preview_requests` publications and the fields declared for them.

Minimal operational meaning:
publish preview requests flowing from editor/tools into preview_runtime so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
rendered preview truth, preview cache ownership, build jobs, diagnostic issue sets.

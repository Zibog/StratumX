# tool_task_requests

Role class: published tooling sidecar fact set

Canonical role:
own long-running work requests that may end in build, preview, validation, release, or assistant tasks.

One data kind law:
this local pack owns only `tool_task_requests` publications and the fields declared for them.

Minimal operational meaning:
publish long-running work requests that may end in build, preview, validation, release, or assistant tasks so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
runtime execution state ownership, final result truth, session/layout truth, hidden scheduler queues.

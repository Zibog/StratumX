# tool_task_results

Role class: published tooling sidecar fact set

Canonical role:
own structured results of long-running tasks without owning their source truth.

One data kind law:
this local pack owns only `tool_task_results` publications and the fields declared for them.

Minimal operational meaning:
publish structured results of long-running tasks without owning their source truth so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
scheduler ownership, task request truth, artifact payload storage, UI notification state.

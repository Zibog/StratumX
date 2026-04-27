# Invalidation

This contract belongs specifically to the tool task requests level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_task_requests`
- dependency shift in `l6.0-authority-core` that changes `task_request_id` semantics
- dependency shift in `l6.0-tool-session` that changes `task_kind` semantics
- supersede, deny, or cancel affecting `task_request_id`
- budget pressure that invalidates disposable outputs of `tool_task_requests` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_task_requests` must explicitly name stale records such as `task_request_id`, `task_kind`, `target_scope`, `priority` instead of rebuilding an unnamed mirror.

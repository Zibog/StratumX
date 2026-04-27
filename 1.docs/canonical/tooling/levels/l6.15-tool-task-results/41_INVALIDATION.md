# Invalidation

This contract belongs specifically to the tool task results level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_task_results`
- dependency shift in `l6.0-authority-core` that changes `task_result_id` semantics
- dependency shift in `l6.0-tool-session` that changes `task_request_id` semantics
- supersede, deny, or cancel affecting `task_result_id`
- budget pressure that invalidates disposable outputs of `tool_task_results` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_task_results` must explicitly name stale records such as `task_result_id`, `task_request_id`, `result_state`, `result_ref` instead of rebuilding an unnamed mirror.

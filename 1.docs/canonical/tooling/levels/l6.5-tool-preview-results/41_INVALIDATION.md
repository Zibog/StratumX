# Invalidation

This contract belongs specifically to the tool preview results level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_preview_results`
- dependency shift in `l6.0-authority-core` that changes `preview_result_id` semantics
- dependency shift in `l6.0-tool-session` that changes `preview_request_id` semantics
- supersede, deny, or cancel affecting `preview_result_id`
- budget pressure that invalidates disposable outputs of `tool_preview_results` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_preview_results` must explicitly name stale records such as `preview_result_id`, `preview_request_id`, `result_ref`, `result_state` instead of rebuilding an unnamed mirror.

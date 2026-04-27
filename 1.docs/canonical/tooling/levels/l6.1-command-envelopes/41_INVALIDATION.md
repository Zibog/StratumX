# Invalidation

This contract belongs specifically to the command envelopes level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `command_envelopes`
- dependency shift in `l6.0-authority-core` that changes `command_envelope_id` semantics
- dependency shift in `l6.0-tool-session` that changes `command_kind` semantics
- dependency shift in `l6.14-tool-task-requests` that changes `target_ref_set` semantics
- supersede, deny, or cancel affecting `command_envelope_id`
- budget pressure that invalidates disposable outputs of `command_envelopes` while preserving authoritative rows

## Invalidation law
Invalidation in `command_envelopes` must explicitly name stale records such as `command_envelope_id`, `command_kind`, `target_ref_set`, `issuer_session_id` instead of rebuilding an unnamed mirror.

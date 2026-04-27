# Invalidation

This contract belongs specifically to the transaction ledger level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `transaction_ledger`
- dependency shift in `l6.0-authority-core` that changes `transaction_id` semantics
- dependency shift in `l6.1-command-envelopes` that changes `command_envelope_id` semantics
- supersede, deny, or cancel affecting `transaction_id`
- budget pressure that invalidates disposable outputs of `transaction_ledger` while preserving authoritative rows

## Invalidation law
Invalidation in `transaction_ledger` must explicitly name stale records such as `transaction_id`, `command_envelope_id`, `before_authority_epoch`, `after_authority_epoch` instead of rebuilding an unnamed mirror.

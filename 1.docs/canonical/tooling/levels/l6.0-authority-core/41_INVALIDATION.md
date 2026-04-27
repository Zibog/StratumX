# Invalidation

This contract belongs specifically to the authority core level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `authority_core`
- dependency shift in `l6.2-transaction-ledger` that changes `authority_epoch` semantics
- dependency shift in `l6.9-budget-runtime` that changes `writer_scope_id` semantics
- supersede, deny, or cancel affecting `authority_epoch`
- budget pressure that invalidates disposable outputs of `authority_core` while preserving authoritative rows

## Invalidation law
Invalidation in `authority_core` must explicitly name stale records such as `authority_epoch`, `writer_scope_id`, `mutation_ticket`, `deny_reason` instead of rebuilding an unnamed mirror.

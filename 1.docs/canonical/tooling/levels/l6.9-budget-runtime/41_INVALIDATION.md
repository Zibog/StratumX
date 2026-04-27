# Invalidation

This contract belongs specifically to the budget runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `budget_runtime`
- dependency shift in `l6.8-cache-plane` that changes `budget_scope_id` semantics
- supersede, deny, or cancel affecting `budget_scope_id`
- budget pressure that invalidates disposable outputs of `budget_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `budget_runtime` must explicitly name stale records such as `budget_scope_id`, `resource_class`, `hard_limit`, `soft_limit` instead of rebuilding an unnamed mirror.

## Operational note
This file remains active and package-specific for `l6.9-budget-runtime` / `41_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

# tool_activation_rules internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| activation_rule_row_id | ActivationRuleRowId | required | canonical storage row identity | stable within the sidecar store |
| activation_rule_id | ActivationRuleId | required | foreign key to the published activation rule | must resolve to a root-level row |
| compiled_predicate_ref | CompiledPredicateRef | optional | compiled predicate used to evaluate the rule quickly | must resolve when predicate compilation exists |
| activation_domain_ref | ActivationDomainRef | required | domain or workspace region where the rule is legal | must resolve through declared refs |
| last_evaluation_cursor | Cursor | optional | cursor of the last completed evaluation | monotonic when present |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_activation_rules`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.

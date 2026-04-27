# Published activation-rule fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| activation_rule_id | ActivationRuleId | required | published identity for an activation rule | stable across rule revisions |
| tool_kind | ToolKind | required | tool class controlled by the rule | must use declared enum |
| required_surface_set | RefOrRefSet | required | surfaces that must exist for the rule to activate | must resolve through declared refs |
| deny_condition_set | RefOrRefSet | required | conditions that suppress activation | must resolve through declared refs |
| priority | ActivationPriority | required | priority used when multiple rules compete | must use declared enum |
| rule_revision | RuleRevision | required | explicit revision of the activation rule | must increase on semantic change |
| fallback_mode | FallbackActivationMode | optional | fallback behavior when primary surfaces are absent | must use declared enum when present |

## Publication law
This file freezes the externally visible publication contract for activation rules. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.

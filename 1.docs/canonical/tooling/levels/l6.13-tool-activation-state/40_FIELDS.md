# Published activation-state fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| activation_state_id | ActivationStateId | required | published identity for an activation-state row | stable while the activation slot exists |
| tool_kind | ToolKind | required | tool class whose state is described | must use declared enum |
| active_rule_ref | ActivationRuleRef | optional | rule currently justifying activation | required when a rule-backed activation is live |
| activation_state | ToolActivationState | required | current activation posture | must use declared enum and transition monotonically |
| focus_binding_ref | FocusRefId | optional | focus binding associated with the active state | present only when focus is coupled |
| state_epoch | Epoch | required | freshness marker for the activation state | advances on semantic state change |

## Publication law
This file freezes the externally visible publication contract for activation state. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.

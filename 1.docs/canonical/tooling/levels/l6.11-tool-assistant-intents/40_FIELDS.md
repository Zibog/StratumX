# Published assistant-intent fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| assistant_intent_id | AssistantIntentId | required | published identity for an assistant intent | stable across proposal/approval/apply flow |
| intent_kind | AssistantIntentKind | required | class of assistant action requested | must use declared enum |
| target_scope | ScopeRef | required | scope the assistant intent applies to | must resolve through declared refs |
| confidence_band | AssistantConfidenceBand | required | confidence band for the assistant intent | must use declared enum |
| issuer_session_id | ToolSessionId | required | session that issued or hosts the intent | must resolve through tool-session publications |
| proposal_ref | ProposalRef | required | proposal that originated the intent | must resolve through assistant runtime publications |
| approval_state | AssistantApprovalState | required | human review state for the intent | must use declared enum and transition monotonically |

## Publication law
This file freezes the externally visible publication contract for assistant intents. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.

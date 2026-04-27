# Published panel-ref fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| panel_ref_id | PanelRefId | required | published identity for a tool panel binding | stable for the lifetime of the panel binding |
| panel_kind | PanelKind | required | kind of panel being referenced | must use declared enum |
| panel_scope | ScopeRef | required | scope where the panel binding is valid | must resolve through declared refs |
| owner_session_id | ToolSessionId | required | session owning the panel binding | must resolve through tool-session publications |
| visibility_state | PanelVisibilityState | required | visible/hidden/docked posture | must use declared enum |
| panel_epoch | Epoch | required | freshness marker for semantic panel changes | advances on semantic change |

## Publication law
This file freezes the externally visible publication contract for tool panel refs. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.

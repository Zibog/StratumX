# Published view-ref fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| view_ref_id | ViewRefId | required | published identity for a tool view binding | stable for the lifetime of the view binding |
| view_kind | ViewKind | required | kind of view being referenced | must use declared enum |
| view_scope | ScopeRef | required | scope where the view is valid | must resolve through declared refs |
| owner_panel_ref | PanelRefId | optional | panel hosting the view | present when a panel owns the view |
| projection_class | ViewProjectionClass | required | projection class used by the view | must use declared enum |
| view_epoch | Epoch | required | freshness marker for semantic view changes | advances on semantic change |

## Publication law
This file freezes the externally visible publication contract for tool view refs. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.

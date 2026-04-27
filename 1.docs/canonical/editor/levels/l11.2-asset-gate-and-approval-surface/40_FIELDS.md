# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| gate_surface_id | GateSurfaceId | active gate/approval surface identity | unique per host |
| gate_queue_ref | GateQueueRef | current queue of pending gate items | typed and explicit |
| selected_gate_item_ref | GateItemRef | currently selected item in the queue | bounded and publishable |
| approval_state_ref | ApprovalStateRef | approval posture for the selected item | finite enum only |
| gate_action_set | GateActionSet | legal approve/reject/escalate actions | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `asset_gate_and_approval_surface` without stealing truth from neighboring levels or lower packages.

# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| graph_editor_session_id | GraphEditorSessionId | active graph editing session | unique per graph host |
| graph_target_ref | GraphTargetRef | graph asset under edit | typed and explicit |
| node_selection_ref_set | NodeSelectionRefSet | selected nodes/edges in the graph | bounded and publishable |
| schema_binding_ref | GraphSchemaBindingRef | schema applied to the graph | must remain typed |
| graph_validation_state | GraphValidationState | current validation posture for the graph | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `graph_authoring_service` without stealing truth from neighboring levels or lower packages.

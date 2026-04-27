# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| inspector_view_id | InspectorViewId | active inspector identity | unique per inspector host |
| inspected_target_ref | TargetRef | current inspected target | explicit and bounded |
| component_editor_set | ComponentEditorSet | component editors visible for the target | must remain typed and ordered |
| header_state | InspectorHeaderState | fixed header values such as Stable ID or Override State | must remain explicit |
| field_edit_session_id | FieldEditSessionId | active field-edit session | must be transaction-visible when mutating |

## Field law
The records above are the minimum editor-owned state needed to drive `inspector_system` without stealing truth from neighboring levels or lower packages.

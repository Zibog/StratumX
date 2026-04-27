# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| ui_suite_session_id | UISuiteSessionId | active UI/HUD suite session | unique per context |
| ui_target_ref | UITargetRef | screen/widget/graph under edit | explicit |
| layout_preview_ref | UILayoutPreviewRef | current UI preview layout | preview-only and replaceable |
| binding_graph_ref | UIBindingGraphRef | current UI binding graph | typed and explicit |
| ui_validation_request_ref | UIValidationRequestRef | pending UI validation/build request | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `ui_hud_authoring_suite` without stealing truth from neighboring levels or lower packages.

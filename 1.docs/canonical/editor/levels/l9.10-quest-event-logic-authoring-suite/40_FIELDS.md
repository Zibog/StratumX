# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| logic_suite_session_id | LogicSuiteSessionId | active quest/event/logic suite session | unique per context |
| quest_graph_ref | QuestGraphRef | current quest/event graph under edit | typed and explicit |
| signal_track_ref | GameplaySignalTrackRef | current gameplay signal track | typed and ordered |
| logic_preview_ref | LogicPreviewRef | preview of quest/event logic outcomes | preview-only and replaceable |
| logic_validation_request_ref | LogicValidationRequestRef | pending validation request for quest/event logic | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `quest_event_logic_authoring_suite` without stealing truth from neighboring levels or lower packages.

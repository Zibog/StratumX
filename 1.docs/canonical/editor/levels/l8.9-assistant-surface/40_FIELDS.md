# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| assistant_surface_id | AssistantSurfaceId | active assistant surface identity | unique per host |
| conversation_session_id | AssistantConversationId | conversation bound to the surface | explicit and bounded |
| proposal_view_ref | ProposalViewRef | proposal view currently shown | must resolve through tooling assistant runtime |
| apply_revert_action_set | ApplyRevertActionSet | visible apply/revert controls | must remain typed and bounded |
| surface_mode | AssistantSurfaceMode | chat/review/plan/execute mode | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `assistant_surface` without stealing truth from neighboring levels or lower packages.

# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| destruction_suite_session_id | DestructionSuiteSessionId | active destruction suite session | unique per context |
| fracture_target_ref | FractureTargetRef | asset/entity under fracture editing | explicit and bounded |
| fracture_pattern_state | FracturePatternState | active fracture pattern settings | typed and explicit |
| destruction_preview_ref | DestructionPreviewRef | current fracture/destruction preview | preview-only and replaceable |
| destruction_contract_ref | DestructionContractRef | current authored destruction contract | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `destruction_fracture_authoring_suite` without stealing truth from neighboring levels or lower packages.

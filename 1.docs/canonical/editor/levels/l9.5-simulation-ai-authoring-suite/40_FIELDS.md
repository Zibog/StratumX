# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| simulation_suite_session_id | SimulationSuiteSessionId | active simulation/AI suite session | unique per context |
| sim_target_scope | SimulationTargetScope | scope under simulation authoring | explicit and bounded |
| ai_profile_ref | AIProfileRef | current AI or population profile under edit | typed and explicit |
| beat_preview_ref | SimulationBeatPreviewRef | current preview of scripted or simulated beats | preview-only and replaceable |
| balance_request_ref | BalanceRequestRef | pending validation/build request for sim balance | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `simulation_ai_authoring_suite` without stealing truth from neighboring levels or lower packages.

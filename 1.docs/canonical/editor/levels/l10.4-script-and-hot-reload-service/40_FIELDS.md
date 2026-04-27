# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| hot_reload_session_id | HotReloadSessionId | active hot reload session | unique per reload wave |
| script_target_ref | ScriptTargetRef | script/module under edit or reload | explicit |
| reload_safety_state | ReloadSafetyState | safe/warn/block posture for the target | finite enum only |
| reload_request_ref | ReloadRequestRef | pending reload request | command-visible |
| reload_result_ref | ReloadResultRef | latest reload result | bounded and publishable |

## Field law
The records above are the minimum editor-owned state needed to drive `script_and_hot_reload_service` without stealing truth from neighboring levels or lower packages.

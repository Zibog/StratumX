# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| playtest_session_id | PlaytestSessionId | active playtest/capture session | unique per host |
| runtime_bind_ref | RuntimeBindRef | current runtime entity or world bind | typed and explicit |
| capture_request_ref | CaptureRequestRef | active screenshot/video/log capture request | command-visible |
| runtime_watch_ref | RuntimeWatchRef | current Runtime Watch projection | typed and explicit |
| playtest_state | PlaytestState | authoring/preview/simulating/broken state | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `playtest_and_capture_operations` without stealing truth from neighboring levels or lower packages.


## Exact inspection and debug tool labels
- Live property watch
- Runtime entity bind
- PIE Attach
- Component state diff
- Override diff viewer
- Broken reference detector
- Missing dependency detector
- Circular reference detector
- Asset usage heatmap
- Editor performance HUD

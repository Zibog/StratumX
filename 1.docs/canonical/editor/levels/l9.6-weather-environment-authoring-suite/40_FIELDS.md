# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| environment_suite_session_id | EnvironmentSuiteSessionId | active environment suite session | unique per context |
| environment_target_scope | EnvironmentTargetScope | weather/environment scope being edited | explicit |
| weather_profile_ref | WeatherProfileRef | current weather/environment profile | typed and explicit |
| lighting_track_ref | LightingTrackRef | current weather/lighting timeline binding | must stay explicit |
| environment_preview_ref | EnvironmentPreviewRef | current environment preview result | preview-only and replaceable |

## Field law
The records above are the minimum editor-owned state needed to drive `weather_environment_authoring_suite` without stealing truth from neighboring levels or lower packages.

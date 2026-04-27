# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| audio_suite_session_id | AudioSuiteSessionId | active audio/voice suite session | unique per context |
| audio_target_ref | AudioTargetRef | asset/entity/dialogue target under edit | explicit |
| emitter_state | AudioEmitterState | active emitter or routing state for authoring | typed and explicit |
| dialogue_binding_ref | DialogueBindingRef | dialogue binding currently authored | typed and ordered |
| audio_preview_ref | AudioPreviewRef | preview result for the active audio target | preview-only and replaceable |

## Field law
The records above are the minimum editor-owned state needed to drive `audio_voice_authoring_suite` without stealing truth from neighboring levels or lower packages.

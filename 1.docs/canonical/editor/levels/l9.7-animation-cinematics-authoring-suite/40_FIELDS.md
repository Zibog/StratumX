# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| cinematics_suite_session_id | CinematicsSuiteSessionId | active animation/cinematics session | unique per context |
| sequence_ref | SequenceRef | current sequence or timeline under edit | explicit and bounded |
| track_binding_set | TrackBindingSet | bound entities/tracks for the active sequence | typed and ordered |
| camera_rig_state | CameraRigState | camera rail/follow/look-at rig state | editor-local and explicit |
| shot_list_ref | ShotListRef | current shot list or blend graph | must remain command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `animation_cinematics_authoring_suite` without stealing truth from neighboring levels or lower packages.


## Exact Timeline / Sequencer track labels
- Transform Track
- Animation Track
- Audio Track
- VFX Track
- Event Track
- Camera Track
- Dialogue Track
- State Track
- Gameplay Signal Track
- Weather/Lighting Track
- AI Scripted Beat Track

## Exact timeline clip field labels
- Start
- End
- Blend In/Out
- Easing
- Bound Entity
- Clip Payload
- Trigger Conditions
- Preview Only / Runtime
- Loop
- Mute
- Locked

## Exact timeline and camera tool labels
- Camera rail
- Follow rig
- Look-at rig
- Shake profile editor
- Shot list
- Blend graph
- Preview scrub
- Event markers
- Cutscene bind validation

# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| world_suite_session_id | WorldSuiteSessionId | active world suite session | unique per open world authoring context |
| cell_view_ref | WorldCellViewRef | current world partition cell view | must remain explicit |
| data_layer_state_ref | DataLayerStateRef | current Data Layers / Layers panel state | editor-local but publishable |
| streaming_preview_ref | StreamingPreviewRef | current streaming preview projection | must resolve through preview/runtime surfaces |
| region_lock_set | RegionLockSet | locked regions for editing | explicit and bounded |

## Field law
The records above are the minimum editor-owned state needed to drive `world_authoring_suite` without stealing truth from neighboring levels or lower packages.


## Exact Data Layers / Layers panel field labels
- Layer Name
- Layer ID
- Type = Editor / Runtime / Debug / Streaming / Mission
- Default Visibility
- Default Loaded
- Color
- Parent Layer
- Lock
- Cook Rule

## Exact Data Layers / Layers panel operations
- Add Selection to Layer
- Remove Selection from Layer
- Toggle Visibility
- Toggle Loaded
- Set Active Edit Layer
- Export Layer Manifest
- Validate Cross-Layer References

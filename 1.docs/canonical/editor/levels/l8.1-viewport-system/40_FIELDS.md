# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| viewport_id | ViewportId | active viewport identity | unique per viewport surface |
| camera_state_ref | CameraStateRef | current camera state for the viewport | must remain explicit |
| navigation_mode | NavigationMode | active navigation scheme | finite enum only |
| preview_frame_ref | PreviewFrameRef | latest frame or preview ref | versioned and replaceable |
| overlay_set_ref | OverlaySetRef | active overlay/gizmo set | must resolve through overlay_and_gizmo_system |

## Field law
The records above are the minimum editor-owned state needed to drive `viewport_system` without stealing truth from neighboring levels or lower packages.

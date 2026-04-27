# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| overlay_set_id | OverlaySetId | active overlay set identity | unique per viewport |
| gizmo_state_id | GizmoStateId | active gizmo state identity | explicit and bounded |
| transform_handle_set | TransformHandleSet | move/rotate/scale handles | must match active tool context |
| snap_visual_state | SnapVisualState | grid/vertex/surface snap visuals | editor-local and explicit |
| overlay_hit_test_ref | OverlayHitTestRef | current overlay hit-test data | replaceable per frame |

## Field law
The records above are the minimum editor-owned state needed to drive `overlay_and_gizmo_system` without stealing truth from neighboring levels or lower packages.

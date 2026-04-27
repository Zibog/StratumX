# tool_scene_intents internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| scene_intent_row_id | SceneIntentRowId | required | canonical storage row identity | stable within the sidecar store |
| scene_intent_id | SceneIntentId | required | foreign key to the published scene intent | must resolve to a root-level row |
| partition_binding_ref | PartitionBindingRef | optional | binding to world-partition/data-layer context | required for region/data-layer constrained intents |
| placement_manifest_ref | PlacementManifestRef | optional | manifest describing object placement or chunk scope | present when placement is materialized |
| intent_resolution_state | SceneIntentResolutionState | required | latest resolution state for the intent row | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_scene_intents`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.

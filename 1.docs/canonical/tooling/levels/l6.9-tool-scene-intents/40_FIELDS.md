# Published scene-intent fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| scene_intent_id | SceneIntentId | required | published identity for a scene intent | stable across request/result flow |
| intent_kind | SceneIntentKind | required | kind of scene/world action requested | must use declared enum |
| target_scene_ref | SceneRef | optional | scene or world subject targeted by the intent | required when the intent addresses an existing scene subject |
| target_region_ref | RegionRef | optional | region/cell target for world-scale scene intents | present only for region-scoped intents |
| issuer_session_id | ToolSessionId | required | session that issued the scene intent | must resolve through tool-session publications |
| intent_cursor | Cursor | required | cursor when the intent entered the scene-intent stream | monotonic within the stream |

## Publication law
This file freezes the externally visible publication contract for scene intents. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.

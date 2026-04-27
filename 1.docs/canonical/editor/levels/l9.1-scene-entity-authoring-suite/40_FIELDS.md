# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| scene_suite_session_id | SceneSuiteSessionId | active scene/entity suite session | unique per authoring context |
| entity_selection_ref_set | EntitySelectionRefSet | entities currently targeted by the suite | explicit and bounded |
| prefab_instance_view_ref | PrefabInstanceViewRef | current prefab/variant/override view | must remain explicit |
| component_list_ref | ComponentListRef | component list backing the Inspector | typed and ordered |
| chunk_save_ref | SceneChunkSaveRef | current Save as Scene Chunk or One File Per Actor work item | must remain command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `scene_entity_authoring_suite` without stealing truth from neighboring levels or lower packages.

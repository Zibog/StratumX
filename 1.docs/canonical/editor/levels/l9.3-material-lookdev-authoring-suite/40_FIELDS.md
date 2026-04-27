# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| lookdev_suite_session_id | LookdevSuiteSessionId | active lookdev session | unique per material editing context |
| material_target_ref | MaterialTargetRef | material or asset under edit | explicit and bounded |
| slot_mapping_state | MaterialSlotMappingState | active material slot mapping posture | must remain typed |
| lookdev_preview_ref | LookdevPreviewRef | current lookdev preview | must resolve through preview runtime |
| shader_variant_request_ref | ShaderVariantRequestRef | pending lookdev/build request | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `material_lookdev_authoring_suite` without stealing truth from neighboring levels or lower packages.
